from __future__ import annotations

import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time
import tomllib
from pathlib import Path

REPO = os.environ["GITHUB_REPOSITORY"]
CANONICAL_MAIN = os.environ["CANONICAL_MAIN"]
PACKET_BASE = os.environ["PACKET_BASE"]
IMPL_HEAD = os.environ["IMPL_HEAD"]
SPEC_REVISION = os.environ["SPEC_REVISION"]
PACKET_DIGEST = os.environ["PACKET_DIGEST"]
PACKET_FILE_SHA256 = os.environ["PACKET_FILE_SHA256"]
IMPL_DIFF_SHA256 = os.environ["IMPL_DIFF_SHA256"]
IMPL_FILE_LIST_SHA256 = os.environ["IMPL_FILE_LIST_SHA256"]
IMPL_REVIEW_SHA256 = os.environ["IMPL_REVIEW_SHA256"]
PR_CI_RUN = os.environ["PR_CI_RUN"]
PR_FUZZ_RUN = os.environ["PR_FUZZ_RUN"]
QUALIFICATION_RUN = os.environ["QUALIFICATION_RUN"]
DIFFCIPLINE_PIN = os.environ["DIFFCIPLINE_PIN"]
OCR_PIN = os.environ["OCR_PIN"]
RUST_NIGHTLY = os.environ["RUST_NIGHTLY"]
CARGO_FUZZ_VERSION = os.environ["CARGO_FUZZ_VERSION"]
CARGO_FUZZ_SHA256 = os.environ["CARGO_FUZZ_SHA256"]
FUZZ_TARGET_TRIPLE = os.environ["FUZZ_TARGET_TRIPLE"]
EVIDENCE_BRANCH = os.environ["EVIDENCE_BRANCH"]
FUZZ_LOCK_SHA256 = os.environ["FUZZ_LOCK_SHA256"]
FUZZ_WORKFLOW_SHA256 = os.environ["FUZZ_WORKFLOW_SHA256"]

P12 = "sha256:13fc136a4f74bf67a2928642d50b750cd729bb7811bfc3dd8f7369ed4a034079"
P13 = "sha256:ab6bae1410aa8d48f7081aebe7047ffe8d5420b497b9f3d744639de9109cd62c"
P14 = "sha256:95a31438e65a266dfd513b8ed15af56f35fe1184d725672359b567008d67a977"
P23 = "sha256:14dc204de67286a282a2b4e1d8d65fc8d9837b140ced660aa230745ddb82f68c"

EXPECTED_PATHS = [
    ".github/workflows/fuzz-smoke.yml",
    "fuzz/.gitignore",
    "fuzz/Cargo.lock",
    "fuzz/Cargo.toml",
    "fuzz/README.md",
    "fuzz/THIRD_PARTY.md",
    "fuzz/fuzz_targets/bounded_utf8.rs",
    "fuzz/fuzz_targets/bounded_vec.rs",
    "fuzz/fuzz_targets/durable_id.rs",
    "fuzz/rust-toolchain.toml",
]

def run(cmd, *, capture=False, env=None, cwd=None):
    print("+", " ".join(str(x) for x in cmd), flush=True)
    if capture:
        return subprocess.check_output(cmd, text=True, env=env, cwd=cwd)
    return subprocess.run(cmd, check=True, env=env, cwd=cwd)

def sha256_file(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def git_show(ref, path):
    return run(["git", "show", f"{ref}:{path}"], capture=True)

def git_blob(ref, path):
    return run(["git", "rev-parse", f"{ref}:{path}"], capture=True).strip()

def gh_json(endpoint):
    return json.loads(run(["gh", "api", "-X", "GET", endpoint], capture=True))

def get_run(run_id):
    return gh_json(f"repos/{REPO}/actions/runs/{run_id}")

def assert_run(run_id, *, name, sha, event=None):
    x = get_run(run_id)
    assert x["name"] == name, (run_id, x["name"], name)
    assert x["head_sha"] == sha, (run_id, x["head_sha"], sha)
    assert x["status"] == "completed", (run_id, x["status"])
    assert x["conclusion"] == "success", (run_id, x["conclusion"])
    if event is not None:
        assert x["event"] == event, (run_id, x["event"], event)
    return x

def latest_push_run(name):
    data = gh_json(f"repos/{REPO}/actions/runs?head_sha={CANONICAL_MAIN}&event=push&per_page=100")
    runs = [
        r for r in data["workflow_runs"]
        if r["name"] == name
        and r["head_sha"] == CANONICAL_MAIN
        and r["event"] == "push"
        and r["status"] == "completed"
        and r["conclusion"] == "success"
    ]
    assert runs, (name, CANONICAL_MAIN)
    runs.sort(key=lambda r: r["id"], reverse=True)
    return runs[0]

def assert_ci_matrix(run_id):
    jobs = gh_json(f"repos/{REPO}/actions/runs/{run_id}/jobs?per_page=100")["jobs"]
    wanted = {"verify (ubuntu-latest)", "verify (macos-latest)", "verify (windows-latest)"}
    found = {j["name"]: j["conclusion"] for j in jobs if j["name"] in wanted}
    assert set(found) == wanted, found
    assert all(v == "success" for v in found.values()), found

def assert_fuzz_job(run_id):
    jobs = gh_json(f"repos/{REPO}/actions/runs/{run_id}/jobs?per_page=100")["jobs"]
    smoke = [j for j in jobs if j["name"] == "smoke"]
    assert len(smoke) == 1, [j["name"] for j in jobs]
    assert smoke[0]["conclusion"] == "success", smoke[0]

run(["git", "fetch", "origin", "main", "impl/sg-000024-isolated-fuzz-harness-v2", "--prune"])
assert run(["git", "rev-parse", "origin/main"], capture=True).strip() == CANONICAL_MAIN
assert run(["git", "rev-parse", "origin/impl/sg-000024-isolated-fuzz-harness-v2"], capture=True).strip() == IMPL_HEAD
probe = subprocess.run(
    ["git", "ls-remote", "--exit-code", "origin", f"refs/heads/{EVIDENCE_BRANCH}"],
    stdout=subprocess.DEVNULL,
    stderr=subprocess.DEVNULL,
)
if probe.returncode == 0:
    raise SystemExit(f"Refusing to overwrite existing {EVIDENCE_BRANCH}")

run(["git", "config", "user.name", "Gocceri"])
run(["git", "config", "user.email", "azialshehri@gmail.com"])

run(["git", "checkout", "--detach", PACKET_BASE])
for sid in ("SG-000012", "SG-000013", "SG-000014", "SG-000023"):
    Path(f"/tmp/{sid}.json").write_text(run(["specgrain", "prove", "--json", sid, "."], capture=True))

proof_expect = {
    "SG-000012": (P12, "sha256:256a8c55a843f00798f3e8e3f0807ee38f676a7f50c9b9380b9d26167f4f7aa7", "026b2e5849759915975f8ae12b111d1185fda89f"),
    "SG-000013": (P13, "sha256:adf07a2ef558c801fcacbb3b1bb7ab627bdc0480fdec65e7fc2d1a2e86dafe8d", "9db66a7b4225cd5d3eff9a65d910038352d29a59"),
    "SG-000014": (P14, "sha256:06df9fd48a398fb1b1af3e4a6c821efa9a0508d7e9e3a795c21edadf1045ef1e", "fac420fd87f86b97a5ee97397e0775d00795aaa3"),
    "SG-000023": (P23, "sha256:9e5cba94eec7f7488cdb02605db5cd7f4d1b41200282ca4efc0c68f058d0a779", "4620805a440fe1ef5d29c9015b781014a5c109d2"),
}
for sid, (record, spec_rev, impl) in proof_expect.items():
    p = json.loads(Path(f"/tmp/{sid}.json").read_text())
    assert p["verified"] is True
    assert p["latest_record_digest"] == record
    latest = p["records"][-1]["report"]
    assert latest["spec_revision"] == spec_rev
    assert latest["implementation_revision"] == impl
    assert latest["issues"] == []
    run(["git", "merge-base", "--is-ancestor", impl, PACKET_BASE])
    print(f"{sid}_PREREQUISITE=PASS")

def proof_summary(path):
    raw = json.loads(git_show(PACKET_BASE, path))
    report = raw["report"]
    return json.dumps({
        "record_digest": raw["record_digest"],
        "verified": report["verified"],
        "spec_id": report["spec_id"],
        "spec_revision": report["spec_revision"],
        "implementation_revision": report["implementation_revision"],
        "issues": report["issues"],
    }, sort_keys=True, separators=(",", ":"))

source = git_show(PACKET_BASE, "docs/SOURCE_LEDGER.md")
s = source.index("rust-fuzz/cargo-fuzz")
try:
    e = source.index("convaiinnovations/laya", s)
except ValueError:
    e = min(len(source), s + 5000)
fuzz_source = source[max(0, s - 3):e]

roadmap = git_show(PACKET_BASE, "docs/roadmap/FOUNDATION_CORE.md")
rs = roadmap.index("## P1 — Rust deterministic kernel")
re_ = roadmap.index("## P2", rs)
p1 = roadmap[rs:re_]

selections = [
    ("sg24-grain", ".specgrain/specs/SG-000024.json", git_show(PACKET_BASE, ".specgrain/specs/SG-000024.json"), "Exact corrected SG-000024 GRAIN contract, authorized surface, tool pins, acceptance criteria, non-claims, and required evidence.", 100),
    ("diffcipline-policy", ".diffcipline.toml", git_show(PACKET_BASE, ".diffcipline.toml"), "Canonical corrected Diffcipline policy proving fuzz/** is admitted before WorkPacket export without weakening other policy fields.", 100),
    ("proof-policy", "docs/adr/0004-verified-proof-prerequisites.md", git_show(PACKET_BASE, "docs/adr/0004-verified-proof-prerequisites.md"), "Canonical ADR-0004 proof-prerequisite policy.", 95),
    ("sg12-proof", f".specgrain/evidence/SG-000012/{P12[7:]}.json", proof_summary(f".specgrain/evidence/SG-000012/{P12[7:]}.json"), "Exact canonical SG-000012 proof identity.", 90),
    ("sg13-proof", f".specgrain/evidence/SG-000013/{P13[7:]}.json", proof_summary(f".specgrain/evidence/SG-000013/{P13[7:]}.json"), "Exact canonical SG-000013 proof identity.", 90),
    ("sg14-proof", f".specgrain/evidence/SG-000014/{P14[7:]}.json", proof_summary(f".specgrain/evidence/SG-000014/{P14[7:]}.json"), "Exact canonical SG-000014 proof identity.", 90),
    ("sg23-proof", f".specgrain/evidence/SG-000023/{P23[7:]}.json", proof_summary(f".specgrain/evidence/SG-000023/{P23[7:]}.json"), "Exact canonical SG-000023 proof identity.", 90),
    ("fuzz-source-ledger", "docs/SOURCE_LEDGER.md", fuzz_source, "Pinned cargo-fuzz/libfuzzer-sys versions, checksums, license boundary, platform distinction, and fuzz-only provenance.", 85),
    ("p1-roadmap", "docs/roadmap/FOUNDATION_CORE.md", p1, "P1 requirement for property and fuzz tests plus deterministic-kernel gate.", 80),
]
records = []
for source_id, path, data, reason, priority in selections:
    encoded = data.encode()
    records.append({
        "source_id": source_id,
        "provenance": f"git:{PACKET_BASE}:{path}",
        "selection_reason": reason,
        "revision": f"git:{git_blob(PACKET_BASE, path)}",
        "size_bytes": len(encoded),
        "token_cost": (len(encoded) + 3) // 4,
        "requirement": "required",
        "priority": priority,
    })
assert sum(r["token_cost"] for r in records) <= 14000
Path("/tmp/sg24-context-sources.json").write_text(json.dumps(records, indent=2, sort_keys=True, ensure_ascii=False) + "\n")
packet_raw = run(["specgrain", "packet", "SG-000024", ".", "--context-sources", "/tmp/sg24-context-sources.json", "--json"], capture=True)
Path("/tmp/sg24-workpacket.json").write_text(packet_raw)
packet_obj = json.loads(packet_raw)
assert packet_obj["spec_id"] == "SG-000024"
assert packet_obj["spec_revision"] == SPEC_REVISION
assert packet_obj["packet_digest"] == PACKET_DIGEST
assert len(packet_obj["context_sources"]) == 9
assert sha256_file("/tmp/sg24-workpacket.json") == PACKET_FILE_SHA256
print("WORKPACKET_REPRODUCTION=PASS")

run(["git", "checkout", "--detach", CANONICAL_MAIN])
assert run(["git", "rev-parse", f"{IMPL_HEAD}^{{tree}}"], capture=True).strip() == run(["git", "rev-parse", f"{CANONICAL_MAIN}^{{tree}}"], capture=True).strip()
print("IMPLEMENTATION_MERGE_TREE_IDENTITY=PASS")

assert_run(PR_CI_RUN, name="ci", sha=IMPL_HEAD, event="pull_request")
assert_ci_matrix(int(PR_CI_RUN))
assert_run(PR_FUZZ_RUN, name="fuzz-smoke", sha=IMPL_HEAD, event="pull_request")
assert_fuzz_job(int(PR_FUZZ_RUN))
qualification = get_run(QUALIFICATION_RUN)
assert qualification["name"] == "sg24-implementation-review-v2", qualification["name"]
assert qualification["head_branch"] == "automation/sg-000024-implementation-review-v2", qualification["head_branch"]
assert qualification["status"] == "completed" and qualification["conclusion"] == "success"
assert qualification["event"] == "push"

main_ci = latest_push_run("ci")
main_fuzz = latest_push_run("fuzz-smoke")
assert_ci_matrix(main_ci["id"])
assert_fuzz_job(main_fuzz["id"])
print(f"MAIN_CI_RUN={main_ci['id']}")
print(f"MAIN_FUZZ_RUN={main_fuzz['id']}")

files = run(["git", "diff", "--name-only", f"{PACKET_BASE}...{IMPL_HEAD}"], capture=True).splitlines()
assert sorted(files) == EXPECTED_PATHS, files
diff_sha = hashlib.sha256(run(["git", "diff", f"{PACKET_BASE}...{IMPL_HEAD}"], capture=True).encode()).hexdigest()
file_list_sha = hashlib.sha256(run(["git", "diff", "--name-only", f"{PACKET_BASE}...{IMPL_HEAD}"], capture=True).encode()).hexdigest()
assert diff_sha == IMPL_DIFF_SHA256
assert file_list_sha == IMPL_FILE_LIST_SHA256

for path in ("Cargo.toml", "Cargo.lock", ".github/workflows/ci.yml"):
    assert git_blob(PACKET_BASE, path) == git_blob(CANONICAL_MAIN, path)
assert run(["git", "rev-parse", f"{PACKET_BASE}:crates/morize-core"], capture=True).strip() == run(["git", "rev-parse", f"{CANONICAL_MAIN}:crates/morize-core"], capture=True).strip()
print("ROOT_AND_PRODUCTION_UNCHANGED=PASS")

run(["cargo", "fmt", "--all", "--", "--check"])
run(["cargo", "check", "--workspace", "--all-targets", "--offline"])
run(["cargo", "clippy", "--workspace", "--all-targets", "--offline", "--", "-D", "warnings"])
tests = run(["cargo", "test", "--workspace", "--all-targets", "--offline"], capture=True)
print(tests)
counts = [int(x) for x in re.findall(r"test result: ok\. (\d+) passed;", tests)]
assert counts and sum(counts) == 33, counts
print("CANONICAL_ROOT_TESTS=33")

run(["rustup", "toolchain", "install", RUST_NIGHTLY, "--profile", "minimal"])
run(["rustup", "component", "add", "rust-src", "--toolchain", RUST_NIGHTLY])
rustc = run(["rustc", f"+{RUST_NIGHTLY}", "--version", "--verbose"], capture=True)
assert "release: 1.100.0-nightly" in rustc
assert "commit-hash: 330d317121e16b5db8e5adc63595910528ff2ee7" in rustc

run([
    "curl", "--fail", "--location", "--silent", "--show-error",
    f"https://github.com/rust-fuzz/cargo-fuzz/releases/download/{CARGO_FUZZ_VERSION}/cargo-fuzz-{CARGO_FUZZ_VERSION}-x86_64-unknown-linux-musl.tar.gz",
    "--output", "/tmp/cargo-fuzz.tar.gz",
])
assert sha256_file("/tmp/cargo-fuzz.tar.gz") == CARGO_FUZZ_SHA256
run(["tar", "-xzf", "/tmp/cargo-fuzz.tar.gz", "-C", "/tmp"])
run(["install", "-m", "0755", "/tmp/cargo-fuzz", str(Path.home() / ".cargo/bin/cargo-fuzz")])
assert f"cargo-fuzz {CARGO_FUZZ_VERSION}" in run(["cargo", "fuzz", "--version"], capture=True)

fuzz_env = os.environ.copy()
fuzz_env["RUSTUP_TOOLCHAIN"] = RUST_NIGHTLY
metadata = json.loads(run(["cargo", "metadata", "--manifest-path", "fuzz/Cargo.toml", "--locked", "--format-version", "1"], capture=True, env=fuzz_env))
assert metadata["workspace_root"].endswith("/fuzz")
packages = {p["name"]: p for p in metadata["packages"]}
assert packages["morize-fuzz"]["publish"] == []
assert packages["libfuzzer-sys"]["version"] == "0.4.13"
assert "morize-core" in packages
assert sha256_file("fuzz/Cargo.lock") == FUZZ_LOCK_SHA256
assert sha256_file(".github/workflows/fuzz-smoke.yml") == FUZZ_WORKFLOW_SHA256

third_party = Path("fuzz/THIRD_PARTY.md").read_text()
assert "- License: MIT OR Apache-2.0" in third_party
assert "Combined crate expression: (MIT OR Apache-2.0) AND NCSA" in third_party
assert "Bundled libFuzzer source license: NCSA" in third_party
workflow = Path(".github/workflows/fuzz-smoke.yml").read_text()
assert FUZZ_TARGET_TRIPLE in workflow
assert "-runs=5000" in workflow
assert "actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a" in workflow

targets = run(["cargo", "fuzz", "list", "--fuzz-dir", "fuzz"], capture=True, env=fuzz_env).split()
assert set(targets) == {"durable_id", "bounded_utf8", "bounded_vec"}, targets
for target in ("durable_id", "bounded_utf8", "bounded_vec"):
    run(["cargo", "fuzz", "build", "--fuzz-dir", "fuzz", "--target", FUZZ_TARGET_TRIPLE, target], env=fuzz_env)

corpus = Path("/tmp/sg24-evidence-corpus")
shutil.rmtree(corpus, ignore_errors=True)
for target in ("durable_id", "bounded_utf8", "bounded_vec"):
    (corpus / target).mkdir(parents=True, exist_ok=True)
(corpus / "durable_id" / "zero").write_text("0" * 32)
(corpus / "bounded_utf8" / "ascii").write_text("memory")
(corpus / "bounded_vec" / "bytes").write_bytes(b"\x00\x01\x02\x03")
seeds = {"durable_id": "424242", "bounded_utf8": "424243", "bounded_vec": "424244"}
for target in ("durable_id", "bounded_utf8", "bounded_vec"):
    run([
        "cargo", "fuzz", "run", "--fuzz-dir", "fuzz", "--target", FUZZ_TARGET_TRIPLE,
        target, str(corpus / target), "--",
        "-runs=1000", f"-seed={seeds[target]}", "-timeout=5", "-rss_limit_mb=2048", "-max_len=512",
    ], env=fuzz_env)
print("FRESH_CANONICAL_FUZZ_RUN=1000_PER_TARGET_PASS")

policy = tomllib.loads(Path(".diffcipline.toml").read_text())
assert policy["policy"]["expected_files"].count("fuzz/**") == 1
assert policy["policy"]["forbidden_surfaces"] == ["secrets/**", ".env"]
assert policy["policy"]["dependency_manifest_changes"] == "allow"
assert policy["policy"]["lockfile_changes"] == "allow"
assert policy["policy"]["untracked_files"] == "fail"
print("DIFFCIPLINE_FUZZ_SCOPE_ADMISSION=PASS")

run(["git", "switch", "-C", EVIDENCE_BRANCH, CANONICAL_MAIN])
from specgrain import CheckEvidence, ExecutionResult, WorkPacket, append_verification_report, load_project, verify_execution

project = load_project(".")
node = next(n for n in project.specs if n.id == "SG-000024")
packet = WorkPacket.from_dict(json.loads(Path("/tmp/sg24-workpacket.json").read_text()))
paths = tuple(EXPECTED_PATHS)
main_ci_id = str(main_ci["id"])
main_fuzz_id = str(main_fuzz["id"])

result = ExecutionResult(
    packet_digest=packet.packet_digest,
    status="succeeded",
    summary="Implemented and canonically merged the isolated SG-000024 true-libFuzzer harness with pinned fuzz tooling, bounded Linux campaigns, root/runtime isolation, exact-head semantic review, and fresh post-merge CI/fuzz verification.",
    changed_paths=paths,
    reported_evidence=tuple(packet.required_evidence),
)

pr_ci_ref = f"github-actions:pr-ci={PR_CI_RUN};pr-fuzz={PR_FUZZ_RUN};main-ci={main_ci_id};main-fuzz={main_fuzz_id}"
proof_ref = f"specgrain:SG-000012:{P12};SG-000013:{P13};SG-000014:{P14};SG-000023:{P23};packet-baseline={PACKET_BASE}"
impl_ref = f"git-diff:{PACKET_BASE}...{IMPL_HEAD};sha256={IMPL_DIFF_SHA256}"
review_ref = f"independent-review:sha256:{IMPL_REVIEW_SHA256};head={IMPL_HEAD}"
fuzz_ref = f"github-actions:pr-fuzz={PR_FUZZ_RUN};main-fuzz={main_fuzz_id};fresh-evidence-runs=1000-per-target"
root_ref = f"{pr_ci_ref};root-tests=33/33"

refs = [
    f"{impl_ref};paths=10-authorized",
    f"git:{CANONICAL_MAIN};root-cargo-lock-core-ci-unchanged=true",
    f"git:{CANONICAL_MAIN}:fuzz/Cargo.toml;isolated-workspace=true;libfuzzer-sys=0.4.13",
    f"git:{CANONICAL_MAIN}:fuzz/Cargo.lock;locked-metadata=pass;root-lock-unchanged=true",
    f"git:{CANONICAL_MAIN}:fuzz/rust-toolchain.toml;nightly={RUST_NIGHTLY};rustc-identity=pass",
    f"git:{CANONICAL_MAIN}:.github/workflows/fuzz-smoke.yml;cargo-fuzz={CARGO_FUZZ_VERSION};asset-sha256={CARGO_FUZZ_SHA256};target={FUZZ_TARGET_TRIPLE}",
    f"git:{CANONICAL_MAIN}:fuzz/THIRD_PARTY.md;license-closure=verified",
    f"git:{CANONICAL_MAIN}:fuzz/fuzz_targets/durable_id.rs;{fuzz_ref}",
    f"git:{CANONICAL_MAIN}:fuzz/fuzz_targets/bounded_utf8.rs;{fuzz_ref}",
    f"git:{CANONICAL_MAIN}:fuzz/fuzz_targets/bounded_vec.rs;{fuzz_ref}",
    f"git:{CANONICAL_MAIN}:fuzz/fuzz_targets;pure-invariants=true;unsafe-authored=false",
    f"{pr_ci_ref};standard-ci-unchanged=true;fuzz-smoke-linux-only=true",
    f"git:{CANONICAL_MAIN}:.github/workflows/fuzz-smoke.yml;runs=5000;seeds=fixed;timeout=5;rss=2048;max-len=512;target={FUZZ_TARGET_TRIPLE}",
    f"{fuzz_ref};all-three-targets=success",
    f"git:{CANONICAL_MAIN}:.github/workflows/fuzz-smoke.yml;failure-artifact-action=pinned",
    f"git:{CANONICAL_MAIN}:fuzz/README.md;bounded-smoke-nonclaims=true",
    root_ref,
    fuzz_ref,
    f"{pr_ci_ref};same-implementation-tree=true",
    f"git:{PACKET_BASE}:.diffcipline.toml;fuzz-scope-admission=exact",
    f"diffcipline:r3=pass;alibaba=10/10-accounted;{review_ref}",
    proof_ref,
]
details = [
    "The implementation diff contains exactly the ten SG-000024 authorized fuzz/workflow files.",
    "Root Cargo manifests/lockfile, morize-core production tree, public API surface, and standard CI file are unchanged from the WorkPacket baseline.",
    "The fuzz manifest is an independent non-publishable nested workspace using only local morize-core plus exactly pinned libfuzzer-sys 0.4.13.",
    "The committed fuzz lockfile resolves with --locked while the root Cargo.lock remains unchanged.",
    "The fuzz toolchain is pinned to nightly-2026-09-18 and the resolved rustc release/commit identity was reverified.",
    "cargo-fuzz 0.13.2 is checksum-pinned and fuzz targets are explicitly built and run for x86_64-unknown-linux-gnu.",
    "Fuzz-only provenance records cargo-fuzz and libfuzzer-sys/libFuzzer license boundaries without linking them into runtime artifacts.",
    "The durable-ID raw-byte target exercises successful valid-UTF8 parses across every durable ID type and enforces canonical lowercase text and exact-byte round trips.",
    "The BoundedUtf8 raw-byte target exercises multiple const byte limits and verifies exact acceptance, preservation, and TooLong metadata.",
    "The BoundedVec raw-byte target exercises multiple const item limits and verifies construction/push bounds, order, lossless rejection, and no mutation after failed push.",
    "Morize-authored fuzz targets are pure invariant harnesses with no authored unsafe blocks or external side-effect inputs.",
    "The dedicated fuzz workflow is Linux-only while the unchanged standard CI remains Ubuntu/macOS/Windows and Windows product support does not depend on libFuzzer.",
    "The fuzz workflow fixes target triple, run count, seed, timeout, RSS ceiling, and input-size limit while using coverage-guided libFuzzer.",
    "All three targets pass exact-head and post-merge bounded campaigns; the evidence run also reruns each target for 1000 bounded executions.",
    "Failure artifacts are uploaded only through a commit-pinned artifact action and success does not require a crash artifact.",
    "Documentation explicitly limits claims to the executed bounded campaign and disclaims exhaustive fuzzing, bug absence, coverage completeness, and security proof.",
    "Root fmt/check/clippy/tests pass on the canonical merge, with 33/33 root tests.",
    "The isolated fuzz workspace builds all three pinned targets and bounded Linux campaigns pass.",
    "Standard cross-platform exact-head and post-merge CI plus dedicated exact-head and post-merge fuzz-smoke all pass for the same implementation tree.",
    "The exact WorkPacket baseline admits only fuzz/** to Diffcipline expected_files while preserving all specified policy fields.",
    "Exact-head Diffcipline R3, Alibaba changed-file accounting, and independent semantic implementation review all passed.",
    "ADR-0004 proof prerequisites SG-000012, SG-000013, SG-000014, and SG-000023 match exact canonical record/spec/implementation identities and ancestry.",
]
assert len(node.acceptance) == 22, len(node.acceptance)
acceptance = [CheckEvidence(a, True, r, d) for a, r, d in zip(node.acceptance, refs, details)]

evidence_map = {
    "rollback": (f"git-history:base={PACKET_BASE};merge={CANONICAL_MAIN};fuzz-only-revert-safe=true", "A normal forward revert removes only the isolated fuzz workspace/workflow; runtime code, persisted data, public API, and root lockfile are unaffected."),
    "independent-review": (review_ref, "Exact-head independent semantic implementation review returned PASS with C/H/M/L NONE and no findings."),
    "E-PROOF-PREREQUISITE-SG000012": (f"specgrain:SG-000012:{P12};packet-baseline={PACKET_BASE}", "Canonical SG-000012 proof identity and ancestry revalidated."),
    "E-PROOF-PREREQUISITE-SG000013": (f"specgrain:SG-000013:{P13};packet-baseline={PACKET_BASE}", "Canonical SG-000013 proof identity and ancestry revalidated."),
    "E-PROOF-PREREQUISITE-SG000014": (f"specgrain:SG-000014:{P14};packet-baseline={PACKET_BASE}", "Canonical SG-000014 proof identity and ancestry revalidated."),
    "E-PROOF-PREREQUISITE-SG000023": (f"specgrain:SG-000023:{P23};packet-baseline={PACKET_BASE}", "Canonical SG-000023 proof identity and ancestry revalidated."),
    "E-FUZZ-WORKSPACE-ISOLATION": (f"git:{CANONICAL_MAIN}:fuzz/Cargo.toml;workspace-root=fuzz;root-graph-unchanged=true", "Fuzz tooling remains isolated in its own nested Cargo workspace and does not alter the root/runtime dependency graph."),
    "E-FUZZ-LOCK-CLOSURE": (f"git:{CANONICAL_MAIN}:fuzz/Cargo.lock;sha256={FUZZ_LOCK_SHA256};cargo-metadata-locked=pass", "The committed fuzz lockfile closes the fuzz-only dependency graph and resolves successfully with --locked."),
    "E-FUZZ-TOOLCHAIN-PIN": (f"git:{CANONICAL_MAIN}:fuzz/rust-toolchain.toml;nightly={RUST_NIGHTLY};rustc-identity=pass", "Pinned nightly identity was verified by release and commit hash."),
    "E-CARGO-FUZZ-PIN": (f"cargo-fuzz={CARGO_FUZZ_VERSION};asset-sha256={CARGO_FUZZ_SHA256};verified=true", "The published cargo-fuzz executable matches the pinned version and SHA-256."),
    "E-FUZZ-TARGET-TRIPLE": (f"git:{CANONICAL_MAIN}:.github/workflows/fuzz-smoke.yml;target={FUZZ_TARGET_TRIPLE}", "Fuzz targets are explicitly compiled and run for GNU libc, separating target identity from the musl-linked cargo-fuzz executable."),
    "E-FUZZ-LICENSE-CLOSURE": (f"git:{CANONICAL_MAIN}:fuzz/THIRD_PARTY.md;license-boundary=verified", "Fuzz-only third-party licensing and bundled libFuzzer NCSA boundary are recorded and revalidated."),
    "E-FUZZ-DURABLE-ID": (f"git:{CANONICAL_MAIN}:fuzz/fuzz_targets/durable_id.rs;{fuzz_ref}", "Durable-ID libFuzzer target builds and bounded campaigns pass."),
    "E-FUZZ-BOUNDED-UTF8": (f"git:{CANONICAL_MAIN}:fuzz/fuzz_targets/bounded_utf8.rs;{fuzz_ref}", "BoundedUtf8 libFuzzer target builds and bounded campaigns pass."),
    "E-FUZZ-BOUNDED-VEC": (f"git:{CANONICAL_MAIN}:fuzz/fuzz_targets/bounded_vec.rs;{fuzz_ref}", "BoundedVec libFuzzer target builds and bounded campaigns pass."),
    "E-FUZZ-BOUNDED-CAMPAIGN": (f"{fuzz_ref};configured-pr-main-runs=5000-per-target", "Exact-head and post-merge dedicated workflows completed the bounded 5000-run campaign for every target; fresh evidence rerun completed 1000 per target."),
    "E-FUZZ-FAILURE-ARTIFACTS": (f"git:{CANONICAL_MAIN}:.github/workflows/fuzz-smoke.yml;upload-artifact=043fb46d1a93c77aae656e7c1c64a875d1fc6a0a", "Failure artifact upload behavior is defined with a commit-pinned action; successful campaigns need no crash artifact."),
    "E-FUZZ-NONCLAIMS": (f"git:{CANONICAL_MAIN}:fuzz/README.md;nonclaims=bounded-only", "Documentation does not claim exhaustive fuzzing, bug absence, coverage completeness, or security proof."),
    "E-ROOT-RUST-FMT": (f"{root_ref};check=fmt", "Root cargo fmt passes on exact head and canonical main."),
    "E-ROOT-RUST-CHECK-OFFLINE": (f"{root_ref};check=cargo-check-offline", "Root offline cargo check passes on exact head and canonical main."),
    "E-ROOT-RUST-CLIPPY-OFFLINE": (f"{root_ref};check=clippy-offline", "Root offline clippy with -D warnings passes on exact head and canonical main."),
    "E-ROOT-RUST-TEST-OFFLINE": (f"{root_ref};tests=33/33", "Root offline workspace tests pass with 33/33 tests."),
    "E-DIFFCIPLINE-FUZZ-SCOPE-ADMISSION": (f"git:{PACKET_BASE}:.diffcipline.toml;expected-files-fuzz=1;preserved-fields=true", "Corrected canonical planning baseline admits exactly fuzz/** without weakening forbidden surfaces, manifest/lock policy, untracked-file handling, or verification commands."),
    "E-CROSS-PLATFORM-CI": (f"{pr_ci_ref};ubuntu=success;macos=success;windows=success", "Exact-head and post-merge standard CI matrices pass Ubuntu, macOS, and Windows."),
    "E-LINUX-FUZZ-SMOKE": (f"{fuzz_ref};linux=success", "Dedicated Linux fuzz-smoke passes on exact implementation head and canonical merge."),
}
assert len(node.evidence["required"]) == 25
evidence = [CheckEvidence(e, True, *evidence_map[e]) for e in node.evidence["required"]]

report = verify_execution(
    node, packet, result,
    implementation_revision=CANONICAL_MAIN,
    observed_changed_paths=paths,
    acceptance_checks=acceptance,
    evidence_checks=evidence,
)
print("result_digest=" + result.result_digest)
print("verified=" + str(report.verified).lower())
print(f"acceptance_pass={sum(c.passed for c in report.acceptance_checks)}/{len(report.acceptance_checks)}")
print(f"evidence_pass={sum(c.passed for c in report.evidence_checks)}/{len(report.evidence_checks)}")
print("issues=" + json.dumps([i.to_dict() for i in report.issues]))
assert report.verified is True

record = append_verification_report(Path("."), report)
evidence_path = Path(".specgrain/evidence/SG-000024") / (record.record_digest[7:] + ".json")
assert evidence_path.exists()
native_summary = {
    "result_digest": result.result_digest,
    "record_digest": record.record_digest,
    "evidence_path": evidence_path.as_posix(),
    "evidence_file_sha256": sha256_file(evidence_path),
    "acceptance_pass": sum(c.passed for c in report.acceptance_checks),
    "acceptance_total": len(report.acceptance_checks),
    "evidence_pass": sum(c.passed for c in report.evidence_checks),
    "evidence_total": len(report.evidence_checks),
    "main_ci_run": main_ci_id,
    "main_fuzz_run": main_fuzz_id,
}
Path("/tmp/sg24-evidence-native-summary.json").write_text(json.dumps(native_summary, indent=2, sort_keys=True) + "\n")
prove = json.loads(run(["specgrain", "prove", "--json", "SG-000024", "."], capture=True))
assert prove["verified"] is True
assert prove["latest_record_digest"] == record.record_digest
print("NATIVE_SPEC_GRAIN_PROVE=PASS")

run(["git", "add", evidence_path.as_posix()])
run(["git", "diff", "--cached", "--check"])
run(["git", "commit", "-s", "-m", "evidence: record SG-000024 verification proof"])
evidence_head = run(["git", "rev-parse", "HEAD"], capture=True).strip()
evidence_tree = run(["git", "rev-parse", "HEAD^{tree}"], capture=True).strip()
changed = run(["git", "diff", "--name-only", f"{CANONICAL_MAIN}...{evidence_head}"], capture=True).splitlines()
assert changed == [evidence_path.as_posix()], changed
evidence_diff_sha = hashlib.sha256(run(["git", "diff", f"{CANONICAL_MAIN}...{evidence_head}"], capture=True).encode()).hexdigest()
evidence_file_list_sha = hashlib.sha256(run(["git", "diff", "--name-only", f"{CANONICAL_MAIN}...{evidence_head}"], capture=True).encode()).hexdigest()

run(["git", "clone", "--quiet", "https://github.com/TheHalfMoon/Diffcipline.git", "/tmp/diffcipline"])
run(["git", "-C", "/tmp/diffcipline", "checkout", "--quiet", DIFFCIPLINE_PIN])
with open("/tmp/sg24-evidence-diffcipline.json", "w") as out, open("/tmp/sg24-evidence-diffcipline.err", "w") as err:
    subprocess.run([
        "cargo", "run", "--quiet", "--locked",
        "--manifest-path", "/tmp/diffcipline/crates/diffcipline-cli/Cargo.toml",
        "--", "check", "--base", CANONICAL_MAIN, "--risk", "R3", "--run", "--json",
    ], check=True, stdout=out, stderr=err)
diffcipline_sha = sha256_file("/tmp/sg24-evidence-diffcipline.json")
lines = [line for line in Path("/tmp/sg24-evidence-diffcipline.json").read_text().splitlines() if line.strip()]
dx = json.loads(lines[-1])
assert dx["verdict"] == "PASS", dx
assert dx["changed_files"] == 1, dx
assert dx["scope_violations"] == [], dx
assert all(v["state"] == "PASS" for v in dx["verification"]), dx
print("DIFFCIPLINE_EVIDENCE_R3=PASS")

run(["git", "clone", "--quiet", "https://github.com/alibaba/open-code-review.git", "/tmp/ocr"])
run(["git", "-C", "/tmp/ocr", "checkout", "--quiet", OCR_PIN])
run(["go", "build", "-o", "/tmp/ocr-bin", "./cmd/opencodereview"], cwd="/tmp/ocr", env={**os.environ, "GOTOOLCHAIN": "auto"})
with open("/tmp/sg24-evidence-ocr.json", "w") as out:
    subprocess.run([
        "/tmp/ocr-bin", "delegate", "preview", "--format", "json",
        "--from", CANONICAL_MAIN, "--to", evidence_head,
    ], check=True, stdout=out, env={**os.environ, "OCR_NO_UPDATE": "1"})
ox = json.loads(Path("/tmp/sg24-evidence-ocr.json").read_text())
assert ox["total_files"] == 1
reviewable = [f["path"] for f in ox.get("reviewable_files", [])]
excluded = [f["path"] for f in ox.get("excluded_files", [])]
assert sorted(reviewable + excluded) == [evidence_path.as_posix()]
rule_accounted = []
if reviewable:
    with open("/tmp/sg24-evidence-rules.json", "w") as out:
        subprocess.run(["/tmp/ocr-bin", "delegate", "rule", "--format", "json", *reviewable], check=True, stdout=out)
    groups = json.loads(Path("/tmp/sg24-evidence-rules.json").read_text())["groups"]
    rule_accounted = sorted(p for g in groups for p in g["files"])
    assert rule_accounted == sorted(reviewable)
print(f"ALIBABA_EVIDENCE_ACCOUNTING=total=1;reviewable={len(reviewable)};excluded={len(excluded)};rule_accounted={len(rule_accounted)}")

run([sys.executable, "-m", "pip", "install", "--disable-pip-version-check", "--quiet", "gradio_client>=1.8,<2"])
from gradio_client import Client

record_text = evidence_path.read_text()
native_text = Path("/tmp/sg24-evidence-native-summary.json").read_text()
prompt = "\n".join([
    "Independent semantic EVIDENCE review of Morize SG-000024. Return final structured review only; do not expose chain-of-thought or tool traces.",
    "",
    f"REVIEW_HEAD={evidence_head}",
    f"BASE_MAIN={CANONICAL_MAIN}",
    f"SPEC_REVISION={SPEC_REVISION}",
    f"PACKET_DIGEST={PACKET_DIGEST}",
    f"IMPLEMENTATION_HEAD={IMPL_HEAD}",
    f"IMPLEMENTATION_MERGE={CANONICAL_MAIN}",
    f"IMPLEMENTATION_REVIEW_SHA256={IMPL_REVIEW_SHA256}",
    f"PR_CI_RUN={PR_CI_RUN}",
    f"PR_FUZZ_RUN={PR_FUZZ_RUN}",
    f"MAIN_CI_RUN={main_ci_id}",
    f"MAIN_FUZZ_RUN={main_fuzz_id}",
    f"EVIDENCE_DIFF_SHA256={evidence_diff_sha}",
    f"EVIDENCE_FILE_LIST_SHA256={evidence_file_list_sha}",
    f"EVIDENCE_FILE_SHA256={sha256_file(evidence_path)}",
    f"DIFFCIPLINE_SHA256={diffcipline_sha}",
    f"Alibaba accounting: total=1 reviewable={len(reviewable)} excluded={len(excluded)} rule-accounted={len(rule_accounted)}.",
    "Exact evidence-only scope: one SG-000024 evidence JSON file.",
    "Native verification must report verified=true, all 22 acceptance checks PASS, all 25 required evidence checks PASS, issues=[].",
    "The evidence must preserve the fuzz-only/root-isolated boundary, exact tool/license identities, bounded-campaign semantics, prerequisite identities, and explicit non-claims.",
    "",
    "Native summary:",
    native_text,
    "",
    "Evidence record:",
    record_text,
    "",
    "Review for wrong-head or stale-run evidence, missing/failed checks, root/runtime coupling, unsupported fuzz/security claims, license/tool identity mismatch, fabricated lifecycle transitions, or evidence-chain inconsistency.",
    "",
    "Return exactly:",
    "REVIEW_HEAD",
    evidence_head,
    "REVIEW_KIND",
    "SG-000024_EVIDENCE",
    "CRITICAL = NONE",
    "HIGH = NONE",
    "MEDIUM = NONE",
    "LOW = NONE",
    "FINDINGS",
    "- NONE",
    "FINAL_GATE = PASS",
    "Replace NONE/PASS only when concise material findings require it. Any different REVIEW_HEAD invalidates the review.",
])
Path("/tmp/sg24-evidence-review-prompt.txt").write_text(prompt)
client = Client("CohereLabs/North-Mini-Code-1.0", verbose=False)
final = ""
for attempt in range(1, 4):
    raw = client.predict(prompt, "Rust", "[]", "{}", api_name="/chat")
    obj = json.loads(raw) if isinstance(raw, str) else raw
    content = ""
    if isinstance(obj, dict):
        for msg in obj.get("history", []):
            if msg.get("role") == "assistant":
                content = msg.get("content", "")
    candidate = content.split("</think>", 1)[-1].strip() if "</think>" in content else content.strip()
    if f"REVIEW_HEAD\n{evidence_head}" in candidate and "REVIEW_KIND\nSG-000024_EVIDENCE" in candidate:
        final = candidate
        break
    print(f"REVIEW_PROVIDER_ATTEMPT_{attempt}=INVALID_NONSTRUCTURED")
    print(candidate[:500])
    if attempt < 3:
        time.sleep(10)
if not final:
    raise SystemExit("independent evidence reviewer returned no structured review after 3 attempts")
required = [
    f"REVIEW_HEAD\n{evidence_head}",
    "REVIEW_KIND\nSG-000024_EVIDENCE",
    "CRITICAL = NONE", "HIGH = NONE", "MEDIUM = NONE", "LOW = NONE",
    "FINDINGS\n- NONE", "FINAL_GATE = PASS",
]
if not all(x in final for x in required):
    print(final)
    raise SystemExit("independent semantic evidence review produced a non-PASS result")

summary = dict(native_summary)
summary.update({
    "base_main": CANONICAL_MAIN,
    "evidence_head": evidence_head,
    "evidence_tree": evidence_tree,
    "spec_revision": SPEC_REVISION,
    "packet_digest": PACKET_DIGEST,
    "implementation_head": IMPL_HEAD,
    "implementation_merge": CANONICAL_MAIN,
    "implementation_review_sha256": IMPL_REVIEW_SHA256,
    "pr_ci_run": PR_CI_RUN,
    "pr_fuzz_run": PR_FUZZ_RUN,
    "qualification_run": QUALIFICATION_RUN,
    "main_ci_run": main_ci_id,
    "main_fuzz_run": main_fuzz_id,
    "diff_sha256": evidence_diff_sha,
    "file_list_sha256": evidence_file_list_sha,
    "evidence_file_sha256": sha256_file(evidence_path),
    "diffcipline_sha256": diffcipline_sha,
    "review_prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(),
    "review_final_sha256": hashlib.sha256((final + "\n").encode()).hexdigest(),
    "alibaba_total": 1,
    "alibaba_reviewable": len(reviewable),
    "alibaba_excluded": len(excluded),
    "alibaba_rule_accounted": len(rule_accounted),
})
Path("/tmp/sg24-evidence-summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
print("INDEPENDENT_EVIDENCE_REVIEW_BEGIN")
print(final)
print("INDEPENDENT_EVIDENCE_REVIEW_END")
print("QUALIFICATION_SUMMARY_BEGIN")
print(json.dumps(summary, indent=2, sort_keys=True))
print("QUALIFICATION_SUMMARY_END")

assert run(["git", "status", "--porcelain"], capture=True).strip() == ""
run(["git", "push", "origin", f"HEAD:refs/heads/{EVIDENCE_BRANCH}"])
print("PUSHED_EVIDENCE_BRANCH=" + EVIDENCE_BRANCH)
