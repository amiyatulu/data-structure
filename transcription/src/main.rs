#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: String,
}

impl DNA {
    pub fn new(dna: &str) -> Self {
        DNA {
            nucleotides: dna.to_string(),
        }
    }

    pub fn into_rna(self) -> RNA {
        let rna_nucleotides: String = self
            .nucleotides
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect();
        RNA {
            nucleotides: rna_nucleotides,
        }
    }
}

fn main() {
    let dna = DNA::new("GCTAATTGC");
    let dna_nucleotides = dna.nucleotides.clone();
    println!("{:?}", dna);
    println!("{:?}", dna_nucleotides);
    println!("{:?}", dna.into_rna())
}
