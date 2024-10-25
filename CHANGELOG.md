# v0.6.2

## Features

* Upgrade to `rkyv` `v0.7.44`

## Features

* Update dependencies

## Bugfixes

* Fix `rkyv` support

# v0.6.0

## Features

* Support for `rkyv` (via the `"rkyv"` feature).

## **Breaking Changes**

* Support for `serde` is now behind a feature (`"serde"`)

# v0.5.0

## **Breaking Changes**

* The quoting rules for a sheet with a single quote in it changes from `'Foo \'bar\''!A1` to
  `'Foo ''bar'''!A1`.  This matches how it works in popular spreadsheet programs

# v0.4.3

## Features

* `IntoIterator` for `A1` and `RangeForCell`
* support for parsing quoted sheet names (i.e., `'My Spreadsheet'!A1`)

## **Breaking Changes**

* `A1Iterator` no longer has a lifetime

# v0.4.2

* implement `Display` for `Error`
