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

Here is a step-by-step list of items to perform. Links refer to an actual migration done on pallet `logion_loc`.

## Prerequisites for the first migration of a pallet
If the pallet has already been migrated at least once, this section can be skipped.
* Create a [migration source file](../pallets/logion_loc/src/migration.rs). It must expose a public `migrate()` function.
* In the [pallet source file](../pallets/logion_loc/src/lib.rs), create the following items:
  * a public `migrate()` function that delegates to the one created at the previous step.
  * a storage to hold the version of the storage:
  ```rust
    #[derive(Encode, Decode, Eq, PartialEq, Debug)]
    pub enum StorageVersion {
      V1,
      V2MakeLocVoid,
    }

    impl Default for StorageVersion {
      fn default() -> StorageVersion {
        return StorageVersion::V2MakeLocVoid;
      }
    }

    /// Storage version
    #[pallet::storage]
    #[pallet::getter(fn pallet_storage_version)]
    pub type PalletStorageVersion<T> = StorageValue<_, StorageVersion, ValueQuery>;
  ```
  * In the [runtime](../runtime/src/lib.rs), add a module `migration` that implements the trait [OnRuntimeUpgrade](https://crates.parity.io/frame_support/traits/trait.OnRuntimeUpgrade.html) and calls the `migrate()` function of the pallet:

  ```rust
  mod migration {
    use super::*;
    use frame_support::traits::OnRuntimeUpgrade;
    use pallet_logion_loc;

    pub struct Upgrade;

    impl OnRuntimeUpgrade for Upgrade {
        fn on_runtime_upgrade() -> Weight {
            pallet_logion_loc::migrate::<Runtime>()
        }
    }
  }
  ```

## All migrations of a pallet.
* In the [runtime](../runtime/src/lib.rs):
  * Increment he value of `spec_version`.
  * If the extrinsics interface is changed, increment also the value of `transaction_version`.

  More info about [runtime versioning](https://docs.substrate.io/v3/runtime/upgrades/#runtime-versioning).
* In the [pallet code](../pallets/logion_loc/src/lib.rs) create a new version in the enum `StorageVersion` and make it the **default** value.
* In [migration.rs](../pallets/logion_loc/src/migration.rs) implement a new module named according to the latest version. This module has to:
  * Define/Override the structure to be migrated from.
  * Define a `migrate()` function that will perform the actual migration.
  * Change the top-level `migrate()` function to call the migration in the latest :
  ```rust
  pub fn migrate<T: Config>() -> Weight {
      do_migrate::<T, _>(StorageVersion::V1, StorageVersion::V2MakeLocVoid, v1::migrate::<T>)
  }
  ```
