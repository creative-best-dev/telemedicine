# Changes

## Unreleased - 2021-xx-xx
- `Quoter::requote` now returns `Option<Vec<u8>>`. [#2613]

[#2613]: https://github.com/actix/actix-web/pull/2613


## 0.5.0-rc.2 - 2022-01-21
- Add `Path::as_str`. [#2590]
- Deprecate `Path::path`. [#2590]

[#2590]: https://github.com/actix/actix-web/pull/2590


## 0.5.0-rc.1 - 2022-01-14
- `Resource` trait now have an associated type, `Path`, instead of the generic parameter. [#2568]
- `Resource` is now implemented for `&mut Path<_>` and `RefMut<Path<_>>`. [#2568]

[#2568]: https://github.com/actix/actix-web/pull/2568


## 0.5.0-beta.4 - 2022-01-04
- `PathDeserializer` now decodes all percent encoded characters in dynamic segments. [#2566]
- Minimum supported Rust version (MSRV) is now 1.54.

[#2566]: https://github.com/actix/actix-net/pull/2566


## 0.5.0-beta.3 - 2021-12-17
- Minimum supported Rust version (MSRV) is now 1.52.


## 0.5.0-beta.2 - 2021-09-09
- Introduce `ResourceDef::join`. [#380]
- Disallow prefix routes with tail segments. [#379]
- Enforce path separators on dynamic prefixes. [#378]
- Improve malformed path error message. [#384]
- Prefix segments now always end with with a segment delimiter or end-of-input. [#2355]
- Prefix segments with trailing slashes define a trailing empty segment. [#2355]
- Support multi-pattern prefixes and joins. [#2356]
- `ResourceDef::pattern` now returns the first pattern in multi-pattern resources. [#2356]
- Support `build_resource_path` on multi-pattern resources. [#2356]
- Minimum supported Rust version (MSRV) is now 1.51.

[#378]: https://github.com/actix/actix-net/pull/378
[#379]: https://github.com/actix/actix-net/pull/379
[#380]: https://github.com/actix/actix-net/pull/380
[#384]: https://github.com/actix/actix-net/pull/384
[#2355]: https://github.com/actix/actix-web/pull/2355
[#2356]: https://github.com/actix/actix-web/pull/2356


## 0.5.0-beta.1 - 2021-07-20
- Fix a bug in multi-patterns where static patterns are interpreted as regex. [#366]
- Introduce `ResourceDef::pattern_iter` to get an iterator over all patterns in a multi-pattern resource. [#373]
- Fix segment interpolation leaving `Path` in unintended state after matching. [#368]
- Fix `ResourceDef` `PartialEq` implementation. [#373]
- Re-work `IntoPatterns` trait, adding a `Patterns` enum. [#372]
- Implement `IntoPatterns` for `bytestring::ByteString`. [#372]
- Rename `Path::{len => segment_count}` to be more descriptive of it's purpose. [#370]
- Rename `ResourceDef::{resource_path => resource_path_from_iter}`. [#371]
- `ResourceDef::resource_path_from_iter` now takes an `IntoIterator`. [#373]
- Rename `ResourceDef::{resource_path_named => resource_path_from_map}`. [#371]
- Rename `ResourceDef::{is_prefix_match => find_match}`. [#373]
- Rename `ResourceDef::{match_path => capture_match_info}`. [#373]
- Rename `ResourceDef::{match_path_checked => capture_match_info_fn}`. [#373]
- Remove `ResourceDef::name_mut` and introduce `ResourceDef::set_name`. [#373]
- Rename `Router::{*_checked => *_fn}`. [#373]
- Return type of `ResourceDef::name` is now `Option<&str>`. [#373]
- Return type of `ResourceDef::pattern` is now `Option<&str>`. [#373]

[#368]: https://github.com/actix/actix-net/pull/368
[#366]: https://github.com/actix/actix-net/pull/366
[#368]: https://github.com/actix/actix-net/pull/368
[#370]: https://github.com/actix/actix-net/pull/370
[#371]: https://github.com/actix/actix-net/pull/371
[#372]: https://github.com/actix/actix-net/pull/372
[#373]: https://github.com/actix/actix-net/pull/373


## 0.4.0 - 2021-06-06
- When matching path parameters, `%25` is now kept in the percent-encoded form; no longer decoded to `%`. [#357]
- Path tail patterns now match new lines (`\n`) in request URL. [#360]
- Fixed a safety bug where `Path` could return a malformed string after percent decoding. [#359]
- Methods `Path::{add, add_static}` now take `impl Into<Cow<'static, str>>`. [#345]

[#345]: https://github.com/actix/actix-net/pull/345
[#357]: https://github.com/actix/actix-net/pull/357
[#359]: https://github.com/actix/actix-net/pull/359
[#360]: https://github.com/actix/actix-net/pull/360


## 0.3.0 - 2019-12-31
- Version was yanked previously. See https://crates.io/crates/actix-router/0.3.0


## 0.2.7 - 2021-02-06
- Add `Router::recognize_checked` [#247]

[#247]: https://github.com/actix/actix-net/pull/247


## 0.2.6 - 2021-01-09
- Use `bytestring` version range compatible with Bytes v1.0. [#246]

[#246]: https://github.com/actix/actix-net/pull/246


## 0.2.5 - 2020-09-20
- Fix `from_hex()` method


## 0.2.4 - 2019-12-31
- Add `ResourceDef::resource_path_named()` path generation method


## 0.2.3 - 2019-12-25
- Add impl `IntoPattern` for `&String`


## 0.2.2 - 2019-12-25
- Use `IntoPattern` for `RouterBuilder::path()`


## 0.2.1 - 2019-12-25
- Add `IntoPattern` trait
- Add multi-pattern resources


## 0.2.0 - 2019-12-07
- Update http to 0.2
- Update regex to 1.3
- Use bytestring instead of string


## 0.1.5 - 2019-05-15
- Remove debug prints


## 0.1.4 - 2019-05-15
- Fix checked resource match


## 0.1.3 - 2019-04-22
- Added support for `remainder match` (i.e "/path/{tail}*")


## 0.1.2 - 2019-04-07
- Export `Quoter` type
- Allow to reset `Path` instance


## 0.1.1 - 2019-04-03
- Get dynamic segment by name instead of iterator.


## 0.1.0 - 2019-03-09
- Initial release
