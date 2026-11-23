# Design

## Unified types

1. Locations
- Users need to be able to:
  - Select places in the `location` field for `events`, `tasks` etc.
  - Utilize existing `AppData` to select previously entered `locations`
  - Use `Room Number` and `Floor Number` etc. precision to distinguish places
- __Solution__:
  ```rust
  Location { address: VerifiedAddress, unit: String, }
  ```

2. Timestamps
- Need to handle 2 cases:
  a. `Time Instant`: Same timestamp universally
    - Such as: Start of F1 race
    __Solution__:
    - `rust`: Use `jiff` crate and store as `jiff::ZonedDateTime`
    - `sql (sqlite)`: Store as `TEXT` that allows `ISO-8601 strings`
      ```sql
      
      ```
  b. `Relative Time`: Constant time with respect to users location
    - Such as: Alarm clock, Sunrise, Operating business hours
    __Solution__:
    - `rust`: Use `jiff` crate and store either as `jiff::civil::Time` or `jiff::civil::Date`
      ```rust

- Resources
  - [official sqlite date & functions docs](https://sqlite.org/lang_datefunc.html/)

3. Text input
- Main concern is to invalidate empty strings
- __Solution__:
  ```rust
  ValidTextInput { s: String }
  ```

