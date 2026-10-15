# moive_importer

> **Name note:** the repository / crate is spelled `moive_importer` (typo for *movie*). Keeping the name for history; treat it as “movie importer”.

Rust CLI that converts a movie list `.txt` into a JSON array. Each movie object has: `disc_number`, `year`, `chinese_title`, `filename`.

## Run

```sh
cargo run -- convert DVDs.txt
```

Or after `cargo build --release`:

```sh
./target/release/moive_importer convert DVDs.txt
```

Output `.json` is written next to the input (same stem). Lines like `1.` set the disc number for following titles. Blank lines are ignored; leading/trailing spaces trimmed.

```sh
cargo test
moive_importer --help
```

Sample data: `DVDs.txt` / `DVDs.json` in this repo.

## License

MIT © 2026 CameliaDev