# Morize Governance

## Current model

Morize begins as a founder-led Apache-2.0 open-source project.

The purpose of this lightweight governance is to make authority visible without creating bureaucracy before a maintainer community exists.

## Founder / lead maintainer

The founder currently owns final repository-level decisions for:

- product direction;
- architecture changes;
- license changes;
- release authorization;
- maintainer admission/removal;
- security embargo/release coordination;
- official Morize branding;
- commercial Morize-operated services.

This does not override third-party licenses, contributor rights, or the Apache-2.0 terms.

## Maintainers

Additional maintainers may be appointed based on sustained, trustworthy contribution.

A future maintainer policy should define:

- repository permissions;
- review expectations;
- release authority;
- security access;
- inactivity/removal;
- conflict-of-interest handling.

Until then, maintainer authority is explicit rather than inferred from contribution volume.

## Technical changes

Normal product changes follow:

`SpecGrain -> bounded implementation -> Diffcipline proof -> review -> canonical merge`.

Material architecture/license/commercial-boundary changes require planning authority before implementation.

## Releases

A Git tag or artifact is not an official release merely because it exists.

Official release authority requires:

- release SpecGrain;
- exact-head qualification;
- required checks/evidence;
- release notes;
- source/license/security closure;
- authorized publication.

## Security

Security reports follow `SECURITY.md`.

Embargoed vulnerability information is shared only with maintainers who need it for remediation/release.

## Contributions

The project uses Apache-2.0 inbound=outbound contribution terms and the Developer Certificate of Origin 1.1 as the initial lightweight contribution attestation.

A CLA is not required by the current plan.

If future financing, corporate structure, or legal requirements motivate a CLA, changing the contribution mechanism requires a public governance decision and does not retroactively rewrite rights without valid agreement.

## Commercial services

Open-source repository governance and Morize-operated commercial service operations are related but distinct.

Commercial pricing, billing, customer contracts, and service operations are not decided by community vote merely because the core is open source.

Open-source technical claims and licensing remain accurate regardless of commercial packaging.

## Changes to governance

This document can evolve as the maintainer community grows.

Governance changes should preserve:

- visible authority;
- contributor rights;
- security response capability;
- predictable release ownership;
- low unnecessary process overhead.
