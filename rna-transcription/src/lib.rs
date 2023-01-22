use phf::phf_map;
#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    strand: Vec<char>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    strand: Vec<char>,
}

const DNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];
const RNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'U'];

const DNA_RNA_MAPPING: phf::Map<char, char> = phf_map! {
    'G' => 'C',
    'C' => 'G',
    'T' => 'A',
    'A' => 'U'
};

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(invalid_nucleotide_index) = dna
            .chars()
            .position(|nucleotide| !DNA_NUCLEOTIDES.contains(&nucleotide))
        {
            Err(invalid_nucleotide_index)
        } else {
            let strand: Vec<char> = dna.chars().collect();

            Ok(Dna { strand })
        }
    }

    pub fn into_rna(self) -> Rna {
        self.into()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(invalid_nucleotide_index) = rna
            .chars()
            .position(|nucleotide| !RNA_NUCLEOTIDES.contains(&nucleotide))
        {
            Err(invalid_nucleotide_index)
        } else {
            let strand: Vec<char> = rna.chars().collect();

            Ok(Rna { strand })
        }
    }
}

impl From<Dna> for Rna {
    fn from(dna: Dna) -> Self {
        let dna: String = dna
            .strand
            .into_iter()
            .map(|nucleotide| DNA_RNA_MAPPING[&nucleotide])
            .collect();
        Rna::new(&dna).expect("Valid DNA should always contain RNA nucleotides.")
    }
}
