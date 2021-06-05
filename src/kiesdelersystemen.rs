use super::*;

impl<K: Debug + Clone + PartialEq + Eq + Hash> ZetelVerdeler<K> {
    pub fn kiesdelers(&self, deler_reeks: RangeInclusive<usize>) -> Vec<(K, usize, usize)> {
        let totaal_stemmen = self.totaal_stemmen();
        self.stem_aantallen
            .iter()
            .filter(|(_partij, stemmen)| {
                **stemmen as f32 / totaal_stemmen >= (self.kiesdrempel_perc / 100.)
            })
            .map(|(partij, stemmen)| {
                deler_reeks
                    .clone()
                    .map(|deler| (partij.clone(), *stemmen * 1000 / deler, *stemmen))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }

    pub fn dhondt(&self) -> HashMap<K, usize> {
        self.kiesdeler_verdeling(1..=self.zetels)
    }

    pub fn imperiali(&self) -> HashMap<K, usize> {
        self.kiesdeler_verdeling(2..=(self.zetels + 1))
    }

    fn kiesdeler_verdeling(&self, deler_reeks: RangeInclusive<usize>) -> HashMap<K, usize> {
        let mut delers = self.kiesdelers(deler_reeks);
        delers.sort_unstable_by_key(|(_partij, deler, stemmen)| (*deler, *stemmen));
        delers.into_iter().rev().take(self.zetels).fold(
            self.partij_empty_hm(),
            |mut acc, (partij, _deler, _stemmen)| {
                *acc.get_mut(&partij).unwrap() += 1;
                acc
            },
        )
    }
}
