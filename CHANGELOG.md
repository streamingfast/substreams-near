# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.11.0-beta.1

* The generated protobuf types now come from [buffa](https://crates.io/crates/buffa) instead of
  `prost`. The wire format is unchanged, but the generated Rust API differs:

  * An optional message field is a `buffa::MessageField<T>` rather than an `Option<T>`. Read it with
    `as_option()` for a reference, `into_option()` to take it by value, and build one with
    `MessageField::some(value)`. There is no `as_ref()`.

  * An enum field is a `buffa::EnumValue<E>` rather than an `i32`. Compare against
    `EnumValue::Known(E::Variant)`, and read the raw discriminant with `to_i32()`. An unrecognised
    value is preserved as `EnumValue::Unknown(i32)`.

  * Every message carries a hidden field for unknown wire data, so a struct literal needs
    `..Default::default()`.

  * `Message::decode` takes `&mut impl Buf` rather than `impl Buf`, and `encoded_len()` returns a
    `u32` rather than a `usize`.

  `Block::hash()` keeps its `Option<String>` signature.

* Requires `substreams` 0.8 and Rust 1.83.

## 0.10.2

* Re-release of `0.10.0` to jump over and 1 year old version that was incorrectly tagged as `v0.10.1` and is now causing dependency resolution problem with Prost update.

## 0.10.0

* Bumped dependencies of `substreams` to 0.6 and `prost` to 0.13 (see [Upgrade notes](https://github.com/streamingfast/substreams-rs/releases/tag/v0.6.0)).

## 0.9.5

### Added

- Added hash() convenience method to the `Block` struct