use super::*;

impl<K: Debug + Clone + PartialEq + Eq + Hash> ZetelVerdeler<K> {
    pub fn kiesdelers(&self, deler_reeks: RangeInclusive<u32>) -> Vec<(K, u32, u32)> {
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

    pub fn dhondt(&self) -> HashMap<K, u32> {
        self.kiesdeler_verdeling(1..=self.zetels)
    }

    pub fn imperiali(&self) -> HashMap<K, u32> {
        self.kiesdeler_verdeling(2..=(self.zetels + 1))
    }

    fn kiesdeler_verdeling(&self, deler_reeks: RangeInclusive<u32>) -> HashMap<K, u32> {
        let mut delers = self.kiesdelers(deler_reeks);
        delers.sort_unstable_by_key(|(_partij, deler, stemmen)| (*deler, *stemmen));
        delers.into_iter().rev().take(self.zetels as usize).fold(
            self.partij_empty_hm(),
            |mut acc, (partij, _deler, _stemmen)| {
                *acc.get_mut(&partij).unwrap() += 1;
                acc
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dhondt() {
        let zv = ZetelVerdelerBuilder::new(&[
            ("Partij A".to_string(), 6500),
            ("Partij B".to_string(), 3800),
            ("Partij C".to_string(), 2300),
        ])
        .zetels(25)
        .finish()
        .unwrap();

        let dzv = zv.dhondt();
        dbg!(&dzv);
        assert_eq!(Some(13), dzv.get("Partij A").cloned());
        assert_eq!(Some(8), dzv.get("Partij B").cloned());
        assert_eq!(Some(4), dzv.get("Partij C").cloned());

        let zv = ZetelVerdelerBuilder::new(&[
            ("Partij A".to_string(), 6500),
            ("Partij B".to_string(), 3800),
            ("Partij C".to_string(), 2300),
        ])
        .zetels(25)
        .kiesdrempel(20.)
        .finish()
        .unwrap();

        let dzv = zv.dhondt();
        dbg!(&dzv);
        assert_eq!(Some(16), dzv.get("Partij A").cloned());
        assert_eq!(Some(9), dzv.get("Partij B").cloned());
        assert_eq!(Some(0), dzv.get("Partij C").cloned());
    }
}
