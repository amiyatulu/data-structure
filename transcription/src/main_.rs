#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<Self, usize> {
        for (i, c) in dna.chars().enumerate() {
            match c {
                'A' | 'C' | 'G' | 'T' => continue,
                _ => return Err(i),
            }
        }
        Ok(DNA {
            nucleotides: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> RNA {
        let rna_nucleotides: String = self.nucleotides.chars().map(|c| {
            match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            }
        }).collect();
        
        RNA {
            nucleotides: rna_nucleotides,
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<Self, usize> {
        for (i, c) in rna.chars().enumerate() {
            match c {
                'A' | 'C' | 'G' | 'U' => continue,
                _ => return Err(i),
            }
        }
        Ok(RNA {
            nucleotides: rna.to_string(),
        })
    }
}

fn main() {
    // Example usage
    let dna = DNA::new("GCTA").unwrap();
    let rna = dna.into_rna();
    println!("{:?}", rna);
}

