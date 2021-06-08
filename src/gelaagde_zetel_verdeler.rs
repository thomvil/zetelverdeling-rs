use super::*;

#[derive(Debug)]
pub struct GelaagdeZetelVerdeler<T, U> {
    pub(crate) kieskringen: HashMap<T, ZetelVerdeler<U>>,
}

impl<T, U> GelaagdeZetelVerdeler<T, U> {
    pub fn totaal_stemmen(&self) -> f32 {
        self.kieskringen
            .values()
            .map(|stemmen| stemmen.totaal_stemmen())
            .sum()
    }
}

impl<T: Clone + Eq + Hash, U> GelaagdeZetelVerdeler<T, U> {
    pub fn totaal_stemmen_per_kieskring(&self) -> HashMap<T, f32> {
        self.kieskringen
            .iter()
            .map(|(kieskring_id, zv)| (kieskring_id.clone(), zv.totaal_stemmen()))
            .collect()
    }
}

/////////////
// Builder //
/////////////

#[derive(Debug, Default)]
pub struct GelaagdeZetelVerdelerBuilder<T, U> {
    kieskringen: HashMap<T, ZetelVerdeler<U>>,
}

impl<T: Eq + Hash, U> GelaagdeZetelVerdelerBuilder<T, U> {
    pub fn new() -> Self {
        Self {
            kieskringen: HashMap::new(),
        }
    }

    pub fn add(mut self, kieskring_naam: T, zv: ZetelVerdeler<U>) -> Self {
        self.kieskringen.insert(kieskring_naam, zv);
        self
    }

    pub fn finish(self) -> GelaagdeZetelVerdeler<T, U> {
        GelaagdeZetelVerdeler {
            kieskringen: self.kieskringen,
        }
    }
}

impl<T: Clone + Eq + Hash, U: Clone + Eq + Hash> GelaagdeZetelVerdelerBuilder<T, U> {
    pub fn from_data(
        data: &[(T, &[(U, u32)], u32, f32)],
    ) -> Result<GelaagdeZetelVerdeler<T, U>, String> {
        data.iter()
            .fold(
                Ok(Self::new()),
                |acc, (kieskring, stem_aantallen, zetels, kiesdrempel)| {
                    ZetelVerdelerBuilder::from_data((*stem_aantallen, *zetels, *kiesdrempel))
                        .finish()
                        .and_then(|zv| acc.map(|gzv| gzv.add(kieskring.clone(), zv)))
                },
            )
            .map(|gzv| gzv.finish())
    }

    pub fn constante_drempel(
        kiesdrempel: f32,
        data: &[(T, u32, &[(U, u32)])],
    ) -> Result<GelaagdeZetelVerdeler<T, U>, String> {
        data.iter()
            .fold(
                Ok(Self::new()),
                |acc, (kieskring, zetels, stem_aantallen)| {
                    ZetelVerdelerBuilder::from_data((*stem_aantallen, *zetels, kiesdrempel))
                        .finish()
                        .and_then(|zv| acc.map(|gzv| gzv.add(kieskring.clone(), zv)))
                },
            )
            .map(|gzv| gzv.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_data() {
        let gzv = GelaagdeZetelVerdelerBuilder::from_data(&[
            (
                "zonder drempel",
                &[("Partij A", 6500), ("Partij B", 3800), ("Partij C", 2300)],
                25,
                0.,
            ),
            (
                "met drempel",
                &[("Partij A", 6500), ("Partij B", 3800), ("Partij C", 2300)],
                25,
                20.,
            ),
        ]);
        assert!(gzv.is_ok());

        let zetels = gzv.unwrap().dhondt();

        assert_eq!(13, *zetels.get(&("zonder drempel", "Partij A")).unwrap());
        assert_eq!(8, *zetels.get(&("zonder drempel", "Partij B")).unwrap());
        assert_eq!(4, *zetels.get(&("zonder drempel", "Partij C")).unwrap());
        assert_eq!(16, *zetels.get(&("met drempel", "Partij A")).unwrap());
        assert_eq!(9, *zetels.get(&("met drempel", "Partij B")).unwrap());
        assert_eq!(0, *zetels.get(&("met drempel", "Partij C")).unwrap());
    }

    #[test]
    fn constante_drempel() {
        let gzv = GelaagdeZetelVerdelerBuilder::constante_drempel(
            20.,
            &[
                (
                    "zonder drempel",
                    25,
                    &[("Partij A", 6500), ("Partij B", 3800), ("Partij C", 2300)],
                ),
                (
                    "met drempel",
                    25,
                    &[("Partij A", 6500), ("Partij B", 3800), ("Partij C", 2300)],
                ),
            ],
        );
        assert!(gzv.is_ok());
    }
}
