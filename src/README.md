# Files
| File         | Purpose |
|--------------|---------|
| address.rs   | Main address generating loop, scalars, points, keys, etc.
| cli.rs       | CLI handling
| constants.rs | General constants
| encode.rs    | Custom `base58` encoding function for `11` byte input
| gui.rs       | GUI handling
| main.rs      | Barebones `main()` that starts `CLI/GUI`
| pattern.rs   | Enum for `Third/First` settings
| regexes.rs   | Regex validation
| speed.rs     | Speed calculation
| state.rs     | `State` struct that holds the stats of a run
| threads.rs   | Available thread calculation

## Thanks
Thanks to [kayabaNerve](https://github.com/kayabaNerve) for teaching me ECC cryptography and Rust.
