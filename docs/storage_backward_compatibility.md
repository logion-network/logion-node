# Introduction
Given that blockchain stored data are serialized/deserialized using the
[SCALE Codec](https://docs.substrate.io/v3/advanced/scale-codec/), any change to the runtime must be carefully
examined to determine whether it's backward-compatible or not.
A change is backward compatible if the new runtime is able to decode data created with the previous version(s) of the runtime.

If not backward-compatible, a migration must be added to the new runtime.

# Storage items
Adding new storage items is backward compatible, provided that following rules are respected:
* Struct are not modified.
* Enums only get new values, added at the end of the list.

# Struct
Adding and/or removing fields is **not** backward compatible.
If the struct is stored in a [Substrate storage item](https://docs.substrate.io/v3/runtime/storage/#storage-items), the whole item is impacted, not only
the single entry that contains the additional or lacks the removed field.

# TypedEnum
It's possible to make a Struct change in time using `TypedEnum` - a new enum entry has to be created each time
the struct changes, and vll versions must be kept (and not re-ordered):

Example: in this example, we needed this struct to get 2 new fields:
So
```rust
pub enum LocVoidInfo<LocId> {
  V1 {
    reason: Vec<u8>,
    replacer: Option<LocLink<LocId>>,
  },
  V2 {
    reason: Vec<u8>,
    first_replacer: LocLink<LocId>,
    second_replacer: LocLink<LocId>,
  }
}
```
is backward compatible with:
```rust
pub enum LocVoidInfo<LocId> {
  V1 {
    reason: Vec<u8>,
    replacer: Option<LocLink<LocId>>,
  }
}
```
The LocInfo bytes created when only `V1` existed will be correctly decoded by the new `LocVoidInfo`containing the V2 struct.

## User-defined types
[TypedEnum](https://polkadot.js.org/docs/api/start/types.extend/#user-defined-enum) are implemented this way:
```json
{
  "LocVoidInfo": {
    "_enum": {
      "V1": {
        "reason": "Vec<u8>",
        "replacer": "Option<LocLink<LocId>>"
      }
    }
  }
}
```

# Storage migration
When a given change is not backward-compatible, it's possible to [migrate storage](https://docs.substrate.io/v3/runtime/upgrades/#storage-migrations) from one version
to the other, by providing one method that will explicitly migrate the data.

