use std::{error::Error, path::Path};
use crate::parser;
use crate::models::Record;

/// Reads a sequence file from `input` and transcribes DNA to RNA by replacing 'T' with 'U'.
/// If a sequence already contains 'U', it is assumed to be RNA and left unchanged.
/// Supports FASTA format. Optionally performs mRNA mode to remove introns.
pub fn run_transcription<P: AsRef<Path>>(input: P, mrna: bool) -> Result<(), Box<dyn Error>> {
    let path = input.as_ref();
    
    // Parse records using FASTA parser (only available parser)
    let records: Vec<Record> = parser::parse_fasta(path)?;

    if records.is_empty() {
        eprintln!("No sequences found in {:?}", path);
        return Ok(());
    }

    for record in records {
        // Header
        println!(">{}{}", record.id, if mrna { "_mRNA" } else { "" });
        let seq = record.seq.to_uppercase();

        // Check for existing RNA
        let has_u = seq.contains('U');

        let processed = if mrna {
            // Remove introns using GT...AG splice sites
            let mut coding = seq.clone();
            
            // Find and remove all introns (GT...AG patterns)
            loop {
                if let Some(gt_pos) = coding.find("GT") {
                    // Look for the first AG after this GT
                    let search_start = gt_pos + 2;
                    if let Some(ag_pos) = coding[search_start..].find("AG") {
                        let ag_absolute = search_start + ag_pos;
                        // Remove the intron including GT and AG
                        coding.drain(gt_pos..ag_absolute + 2);
                    } else {
                        // No matching AG found, stop processing
                        break;
                    }
                } else {
                    // No more GT sites found
                    break;
                }
            }
            
            // Transcribe DNA bases if needed
            if has_u {
                coding
            } else {
                coding.chars().map(|c| if c == 'T' { 'U' } else { c }).collect()
            }
        } else if has_u {
            // RNA sequence, no changes
            seq.clone()
        } else {
            // Standard transcription
            seq.chars().map(|c| if c == 'T' { 'U' } else { c }).collect()
        };

        println!("{}", processed);

        if !mrna && has_u {
            eprintln!("Sequence '{}' appears to be RNA; no transcription performed", record.id);
        } else if mrna {
            eprintln!("Sequence '{}' introns removed and {}transcribed",
                      record.id,
                      if has_u { "RNA mode: already RNA, " } else { "standard " });
        }
    }

    Ok(())
}