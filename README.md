# FASTA Parser

A command line FASTA parser and writer written in Rust.

## Usage

```bash
cargo run -- <input_file> [<output_file>] [--gc] [--stats] [--transcribe] [--mrna]
```

## Features

- Supports **csv**, **tsv**, **xml**, and **json** inputs/outputs!

### Optional Flags

- `--gc` - Gives the GC%
- `--stats` - Generates length & GC% statistics and plots
- `--transcribe` - Transcribes DNA to RNA (T→U)
- `--mrna` - Removes introns during transcription using GT...AG splice sites