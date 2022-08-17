pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        let allergies: Vec<Allergen> = vec![Cats, Pollen, Chocolate, Tomatoes, Strawberries, Shellfish, Peanuts, Eggs];
        // allergy score - highest score allergy
        // if allergy score still greater than highest score allergy -- repeat
        // find allergy score less than provided allergy score
        // subract traversal while not negative and greater than 0 
        let mut results: Vec<Allergen> = vec![];
        let mut allergy_score = self.score;
        while allergy_score > 0 {
            match allergies.iter().position(|allergen| allergy_score >= *allergen as u32) {
                Some(index) => {
                    allergy_score -= allergies[index] as u32;
                    if allergy_score <= allergies[index] as u32 {
                        results.push(allergies[index]);
                    }
                },
                _ => unreachable!()
            }
        }
        results
    }
}
