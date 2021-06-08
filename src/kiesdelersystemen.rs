use super::*;

impl<K: Clone> ZetelVerdeler<K> {
    fn kiesdelers(&self, deler_reeks: RangeInclusive<u32>) -> Vec<(K, u32, u32)> {
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
}

impl<K: Clone + Eq + Hash> ZetelVerdeler<K> {
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

impl<T: Clone + Eq + Hash, U: Clone + Eq + Hash> GelaagdeZetelVerdeler<T, U> {
    pub fn dhondt(&self) -> ZetelVerdeling<T, U> {
        self.kieskringen
            .iter()
            .map(move |(kieskring_naam, zv)| {
                zv.dhondt().into_iter().map(move |(partij_naam, zetels)| {
                    ((kieskring_naam.clone(), partij_naam), zetels)
                })
            })
            .flatten()
            .collect()
    }

    pub fn dhondt_per_kieskring(&self) -> HashMap<T, HashMap<U, u32>> {
        self.kieskringen
            .iter()
            .map(|(kieskring_naam, zv)| (kieskring_naam.clone(), zv.dhondt()))
            .collect()
    }

    pub fn imperiali(&self) -> ZetelVerdeling<T, U> {
        self.kieskringen
            .iter()
            .map(move |(kieskring_naam, zv)| {
                zv.imperiali()
                    .into_iter()
                    .map(move |(partij_naam, zetels)| {
                        ((kieskring_naam.clone(), partij_naam), zetels)
                    })
            })
            .flatten()
            .collect()
    }

    pub fn imperiali_per_kieskring(&self) -> HashMap<T, HashMap<U, u32>> {
        self.kieskringen
            .iter()
            .map(|(kieskring_naam, zv)| (kieskring_naam.clone(), zv.imperiali()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dhondt() {
        let zv = ZetelVerdelerBuilder::new(&[
            ("Partij A", 6500),
            ("Partij B", 3800),
            ("Partij C", 2300),
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
            ("Partij A", 6500),
            ("Partij B", 3800),
            ("Partij C", 2300),
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

    #[test]
    fn dhondt_gelaagd() {
        let gzv = GelaagdeZetelVerdelerBuilder::default()
            .add(
                "zonder drempel",
                ZetelVerdelerBuilder::new(&[
                    ("Partij A", 6500),
                    ("Partij B", 3800),
                    ("Partij C", 2300),
                ])
                .zetels(25)
                .finish()
                .unwrap(),
            )
            .add(
                "met drempel",
                ZetelVerdelerBuilder::new(&[
                    ("Partij A", 6500),
                    ("Partij B", 3800),
                    ("Partij C", 2300),
                ])
                .zetels(25)
                .kiesdrempel(20.)
                .finish()
                .unwrap(),
            )
            .finish();
        let zetels = gzv.dhondt();

        dbg!(&zetels);

        assert_eq!(13, zetels.get("zonder drempel", "Partij A").unwrap());
        assert_eq!(8, zetels.get("zonder drempel", "Partij B").unwrap());
        assert_eq!(4, zetels.get("zonder drempel", "Partij C").unwrap());
        assert_eq!(16, zetels.get("met drempel", "Partij A").unwrap());
        assert_eq!(9, zetels.get("met drempel", "Partij B").unwrap());
        assert_eq!(0, zetels.get("met drempel", "Partij C").unwrap());
    }
}
