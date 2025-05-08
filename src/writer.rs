use crate::models::Record;
use std::io::{self, Write};

/// FASTA
pub fn write_fasta<W: Write>(w: &mut W, records: &[Record]) -> io::Result<()> {
    for rec in records {
        writeln!(w, ">{}", rec.id)?;
        writeln!(w, "{}", rec.seq)?;
    }
    Ok(())
}

/// JSON
pub fn write_json<W: Write>(w: &mut W, records: &[Record]) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(w, records)
}

/// CSV
pub fn write_csv<W: Write>(w: &mut W, records: &[Record]) -> csv::Result<()> {
    let mut wtr = csv::Writer::from_writer(w);
    wtr.write_record(&["id", "sequence"])?;
    for rec in records {
        wtr.write_record(&[&rec.id, &rec.seq])?;
    }
    wtr.flush()?;
    Ok(())
}

/// TSV
pub fn write_tsv<W: Write>(w: &mut W, records: &[Record]) -> csv::Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(w);
    wtr.write_record(&["id", "sequence"])?;
    for rec in records {
        wtr.write_record(&[&rec.id, &rec.seq])?;
    }
    wtr.flush()?;
    Ok(())
}

/// XML
pub fn write_xml<W: Write>(w: &mut W, records: &[Record]) -> io::Result<()> {
    writeln!(w, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
    writeln!(w, "<records>")?;
    for rec in records {
        writeln!(w, "  <record>")?;
        writeln!(w, "    <id>{}</id>", rec.id)?;
        writeln!(w, "    <sequence>{}</sequence>", rec.seq)?;
        writeln!(w, "  </record>")?;
    }
    writeln!(w, "</records>")?;
    Ok(())
}