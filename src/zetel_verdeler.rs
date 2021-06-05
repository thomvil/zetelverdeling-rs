use super::*;

#[derive(Debug)]
pub struct ZetelVerdeler<K: Debug + Clone + PartialEq + Eq + Hash> {
    pub stem_aantallen: HashMap<K, usize>,
    pub kiesdrempel_perc: f32,
    pub zetels: usize,
}

impl<K:Debug +  Clone + PartialEq + Eq + Hash> ZetelVerdeler<K> {
    pub fn totaal_stemmen(&self) -> f32 {
        self.stem_aantallen
            .values()
            .map(|stemmen| *stemmen as f32)
            .sum()
    }

    pub fn partij_empty_hm(&self) -> HashMap<K, usize> {
        self.stem_aantallen
            .iter()
            .fold(HashMap::new(), |mut acc, (partij, _)| {
                acc.entry(partij.clone()).or_insert(0);
                acc
            })
    }
}
