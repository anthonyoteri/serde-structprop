# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v0.1.2](https://github.com/anthonyoteri/serde-structprop/compare/6db14b5356b25d25eeff5c3c94e506c06a671ec3..v0.1.2) - 2026-05-12
#### Bug Fixes
- (**release**) drop cargo build from pre_bump_hooks (#21) - ([6db14b5](https://github.com/anthonyoteri/serde-structprop/commit/6db14b5356b25d25eeff5c3c94e506c06a671ec3)) - [@anthonyoteri](https://github.com/anthonyoteri)

- - -

## [v0.1.1](https://github.com/anthonyoteri/serde-structprop/compare/a4e598c415086d1ede3d47a287fe0dcfaefb346d..v0.1.1) - 2026-05-12
#### Bug Fixes
- add v prefix to cog tags to match release.yml trigger (#17) - ([080bd5e](https://github.com/anthonyoteri/serde-structprop/commit/080bd5e176064340f115295fc8e40ca84aa59df6)) - [@anthonyoteri](https://github.com/anthonyoteri)
- write release notes to /tmp to avoid cargo publish dirty check (#18) - ([a4e598c](https://github.com/anthonyoteri/serde-structprop/commit/a4e598c415086d1ede3d47a287fe0dcfaefb346d)) - [@anthonyoteri](https://github.com/anthonyoteri)

- - -

## [0.1.0](https://github.com/anthonyoteri/serde-structprop/compare/fe9b010d2510bf5d051f48d554cd426c597332ba..0.1.0) - 2026-05-12
#### Features
- improve Cargo.toml metadata and make serde derive optional (#3) - ([e3c726e](https://github.com/anthonyoteri/serde-structprop/commit/e3c726ec64cc2cb6bd14b962780419825c4aa5a6)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add cocogitto for conventional commit enforcement and changelog generation (#7) - ([5279fa3](https://github.com/anthonyoteri/serde-structprop/commit/5279fa3416bfa74404f25f46209131e0d7811821)) - [@anthonyoteri](https://github.com/anthonyoteri)
- initial implementation of serde-structprop - ([fe9b010](https://github.com/anthonyoteri/serde-structprop/commit/fe9b010d2510bf5d051f48d554cd426c597332ba)) - [@anthonyoteri](https://github.com/anthonyoteri)
#### Bug Fixes
- resolve all rustdoc broken intra-doc links (#14) - ([0fdfaa5](https://github.com/anthonyoteri/serde-structprop/commit/0fdfaa5bff3b8a69fb0f3a58d6c5e82e0e885e88)) - [@anthonyoteri](https://github.com/anthonyoteri)
- remove direct push to main from changelog CI job (#12) - ([37e299b](https://github.com/anthonyoteri/serde-structprop/commit/37e299bdbc9d70bd0ba6c77172ca7b8a8188303d)) - [@anthonyoteri](https://github.com/anthonyoteri)
- enforce string-only map keys, reject non-null unit values, broaden proptests (#11) - ([a4c0f04](https://github.com/anthonyoteri/serde-structprop/commit/a4c0f045c46e4d0ffbef2dadf87d87f830ccb424)) - [@anthonyoteri](https://github.com/anthonyoteri)
- address critical bugs found in adversarial code review (#10) - ([13a974c](https://github.com/anthonyoteri/serde-structprop/commit/13a974cc8d039ecda614cd19af823c9fd5d89278)) - [@anthonyoteri](https://github.com/anthonyoteri)
- resolve clippy lint errors in integration tests - ([53812af](https://github.com/anthonyoteri/serde-structprop/commit/53812afa592780e9aff3144d40e008328abc5ad2)) - [@anthonyoteri](https://github.com/anthonyoteri)
- correct array/object indentation in write_kv serializer - ([a233254](https://github.com/anthonyoteri/serde-structprop/commit/a233254f15cfc170bff6c0d76389cfed51ff3737)) - [@anthonyoteri](https://github.com/anthonyoteri)
#### Documentation
- add CONTRIBUTING.md - ([a1acaf0](https://github.com/anthonyoteri/serde-structprop/commit/a1acaf0b48231c998066fdaa1d029b788dede7c3)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add CI status badge to README - ([4d6bb93](https://github.com/anthonyoteri/serde-structprop/commit/4d6bb93f7cdbcb80b85c34d944c14a0732ed3625)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add README with format overview, type mapping, and usage examples - ([d025b9a](https://github.com/anthonyoteri/serde-structprop/commit/d025b9a7a8fffb53cc68dff9cbd91b810bc83a7d)) - [@anthonyoteri](https://github.com/anthonyoteri)
#### Tests
- add proptest property-based round-trip tests - ([eef4b57](https://github.com/anthonyoteri/serde-structprop/commit/eef4b579f1d3602751488d94cdaa6019526372fb)) - [@anthonyoteri](https://github.com/anthonyoteri)
#### Miscellaneous
- (**deps**) bump actions/checkout from 4 to 6 - ([1454fa1](https://github.com/anthonyoteri/serde-structprop/commit/1454fa1d36691bdcf650349c5278c892856eb3e1)) - dependabot[bot]
- update release workflow to use release PR + push-tag (#15) - ([fbad868](https://github.com/anthonyoteri/serde-structprop/commit/fbad868ae004fa86a98a73acae462bc6b7d5de0d)) - [@anthonyoteri](https://github.com/anthonyoteri)
- dual-license under MIT OR Apache-2.0 (#13) - ([9354665](https://github.com/anthonyoteri/serde-structprop/commit/9354665f5630852ed2532f7a14b22b9dc737aacc)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add GitHub issue templates and PR template - ([40a55ba](https://github.com/anthonyoteri/serde-structprop/commit/40a55bae1e2276d2a8cf8676d3b1f37141b01844)) - [@anthonyoteri](https://github.com/anthonyoteri)
- deny clippy::all and clippy::pedantic in lib.rs - ([e4f33e4](https://github.com/anthonyoteri/serde-structprop/commit/e4f33e4480ff478d7fc1d38ff0147e4f86f12367)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add MIT license - ([7bf5b3d](https://github.com/anthonyoteri/serde-structprop/commit/7bf5b3d94f78ea50d5d41dc553c5937c576c463c)) - [@anthonyoteri](https://github.com/anthonyoteri)
- enable pedantic clippy and deny missing_docs in lib.rs - ([d5e4f54](https://github.com/anthonyoteri/serde-structprop/commit/d5e4f54b8576dfc87344d797287c61c3d69ac0b3)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add .gitignore and remove target/ from tracking - ([f34a64d](https://github.com/anthonyoteri/serde-structprop/commit/f34a64d418762fba604c9ac211f472fb6ab3ac12)) - [@anthonyoteri](https://github.com/anthonyoteri)

- - -

## [0.1.0](https://github.com/anthonyoteri/serde-structprop/compare/fe9b010d2510bf5d051f48d554cd426c597332ba..0.1.0) - 2026-05-12
#### Features
- improve Cargo.toml metadata and make serde derive optional (#3) - ([e3c726e](https://github.com/anthonyoteri/serde-structprop/commit/e3c726ec64cc2cb6bd14b962780419825c4aa5a6)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add cocogitto for conventional commit enforcement and changelog generation (#7) - ([5279fa3](https://github.com/anthonyoteri/serde-structprop/commit/5279fa3416bfa74404f25f46209131e0d7811821)) - [@anthonyoteri](https://github.com/anthonyoteri)
- initial implementation of serde-structprop - ([fe9b010](https://github.com/anthonyoteri/serde-structprop/commit/fe9b010d2510bf5d051f48d554cd426c597332ba)) - [@anthonyoteri](https://github.com/anthonyoteri)
#### Bug Fixes
- resolve all rustdoc broken intra-doc links (#14) - ([0fdfaa5](https://github.com/anthonyoteri/serde-structprop/commit/0fdfaa5bff3b8a69fb0f3a58d6c5e82e0e885e88)) - [@anthonyoteri](https://github.com/anthonyoteri)
- remove direct push to main from changelog CI job (#12) - ([37e299b](https://github.com/anthonyoteri/serde-structprop/commit/37e299bdbc9d70bd0ba6c77172ca7b8a8188303d)) - [@anthonyoteri](https://github.com/anthonyoteri)
- enforce string-only map keys, reject non-null unit values, broaden proptests (#11) - ([a4c0f04](https://github.com/anthonyoteri/serde-structprop/commit/a4c0f045c46e4d0ffbef2dadf87d87f830ccb424)) - [@anthonyoteri](https://github.com/anthonyoteri)
- address critical bugs found in adversarial code review (#10) - ([13a974c](https://github.com/anthonyoteri/serde-structprop/commit/13a974cc8d039ecda614cd19af823c9fd5d89278)) - [@anthonyoteri](https://github.com/anthonyoteri)
- resolve clippy lint errors in integration tests - ([53812af](https://github.com/anthonyoteri/serde-structprop/commit/53812afa592780e9aff3144d40e008328abc5ad2)) - [@anthonyoteri](https://github.com/anthonyoteri)
- correct array/object indentation in write_kv serializer - ([a233254](https://github.com/anthonyoteri/serde-structprop/commit/a233254f15cfc170bff6c0d76389cfed51ff3737)) - [@anthonyoteri](https://github.com/anthonyoteri)
#### Documentation
- add CONTRIBUTING.md - ([a1acaf0](https://github.com/anthonyoteri/serde-structprop/commit/a1acaf0b48231c998066fdaa1d029b788dede7c3)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add CI status badge to README - ([4d6bb93](https://github.com/anthonyoteri/serde-structprop/commit/4d6bb93f7cdbcb80b85c34d944c14a0732ed3625)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add README with format overview, type mapping, and usage examples - ([d025b9a](https://github.com/anthonyoteri/serde-structprop/commit/d025b9a7a8fffb53cc68dff9cbd91b810bc83a7d)) - [@anthonyoteri](https://github.com/anthonyoteri)
#### Tests
- add proptest property-based round-trip tests - ([eef4b57](https://github.com/anthonyoteri/serde-structprop/commit/eef4b579f1d3602751488d94cdaa6019526372fb)) - [@anthonyoteri](https://github.com/anthonyoteri)
#### Miscellaneous
- (**deps**) bump actions/checkout from 4 to 6 - ([1454fa1](https://github.com/anthonyoteri/serde-structprop/commit/1454fa1d36691bdcf650349c5278c892856eb3e1)) - dependabot[bot]
- dual-license under MIT OR Apache-2.0 (#13) - ([9354665](https://github.com/anthonyoteri/serde-structprop/commit/9354665f5630852ed2532f7a14b22b9dc737aacc)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add GitHub issue templates and PR template - ([40a55ba](https://github.com/anthonyoteri/serde-structprop/commit/40a55bae1e2276d2a8cf8676d3b1f37141b01844)) - [@anthonyoteri](https://github.com/anthonyoteri)
- deny clippy::all and clippy::pedantic in lib.rs - ([e4f33e4](https://github.com/anthonyoteri/serde-structprop/commit/e4f33e4480ff478d7fc1d38ff0147e4f86f12367)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add MIT license - ([7bf5b3d](https://github.com/anthonyoteri/serde-structprop/commit/7bf5b3d94f78ea50d5d41dc553c5937c576c463c)) - [@anthonyoteri](https://github.com/anthonyoteri)
- enable pedantic clippy and deny missing_docs in lib.rs - ([d5e4f54](https://github.com/anthonyoteri/serde-structprop/commit/d5e4f54b8576dfc87344d797287c61c3d69ac0b3)) - [@anthonyoteri](https://github.com/anthonyoteri)
- add .gitignore and remove target/ from tracking - ([f34a64d](https://github.com/anthonyoteri/serde-structprop/commit/f34a64d418762fba604c9ac211f472fb6ab3ac12)) - [@anthonyoteri](https://github.com/anthonyoteri)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).