A command line FASTA parser and writer written in Rust.

Usage:

cargo run -- <input_file> [<output_file>] [--gc] [--stats] [--transcribe] [--mrna]

Supports csv, tsv, xml, and json inputs/outputs!

The optional --gc flag gives the gc%.
The optional --stats flag generates length & GC% statistics and plots.
The optional --transcribe flag transcribes DNA to RNA (T->U).
The optional --mrna flag removes introns during transcription using GT...AG splice sites.
