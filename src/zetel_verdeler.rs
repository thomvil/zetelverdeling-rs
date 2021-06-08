use super::*;

#[derive(Clone, Debug)]
pub struct ZetelVerdeler<K> {
    pub(crate) stem_aantallen: HashMap<K, u32>,
    pub(crate) kiesdrempel_perc: f32,
    pub(crate) zetels: u32,
}

impl<K> ZetelVerdeler<K> {
    pub fn totaal_stemmen(&self) -> f32 {
        self.stem_aantallen
            .values()
            .map(|stemmen| *stemmen as f32)
            .sum()
    }
}

impl<K: Clone + Eq + Hash> ZetelVerdeler<K> {
    pub(crate) fn partij_empty_hm(&self) -> HashMap<K, u32> {
        self.stem_aantallen
            .iter()
            .fold(HashMap::new(), |mut acc, (partij, _)| {
                acc.entry(partij.clone()).or_insert(0);
                acc
            })
    }
}

/////////////
// Builder //
/////////////

pub struct ZetelVerdelerBuilder<K> {
    stem_aantallen: HashMap<K, u32>,
    kiesdrempel_perc: Option<f32>,
    zetels: Option<u32>,
}

impl<K> ZetelVerdelerBuilder<K> {
    pub fn zetels(mut self, zetels: u32) -> Self {
        self.zetels = Some(zetels);
        self
    }

    pub fn kiesdrempel(mut self, kiesdrempel_perc: f32) -> Self {
        self.kiesdrempel_perc = Some(kiesdrempel_perc);
        self
    }

    pub fn finish(self) -> Result<ZetelVerdeler<K>, String> {
        if self.stem_aantallen.is_empty() {
            return Err(String::from(
                "Specifieer partijen met hun aantallen stemmen",
            ));
        }
        if let Some(kd) = self.kiesdrempel_perc {
            if kd >= 50. {
                return Err(format!("'{}' is geen geldige kiesdrempel", kd));
            }
        }
        match self.zetels {
            None => return Err(String::from("Specifieer een aantal te verdelen zetels")),
            Some(z) if z < 2 => return Err(String::from("Te weinig zetels om te verdelen")),
            _ => (),
        }
        Ok(ZetelVerdeler {
            stem_aantallen: self.stem_aantallen,
            kiesdrempel_perc: self.kiesdrempel_perc.unwrap_or(0.),
            zetels: self.zetels.unwrap(),
        })
    }
}

impl<K: Clone + Eq + Hash> ZetelVerdelerBuilder<K> {
    pub fn new(stem_aantallen: &[(K, u32)]) -> Self {
        Self {
            stem_aantallen: stem_aantallen.iter().cloned().collect(),
            kiesdrempel_perc: None,
            zetels: None,
        }
    }

    pub fn from_data((stem_aantallen, zetels, kiesdrempel): (&[(K, u32)], u32, f32)) -> Self {
        Self::new(stem_aantallen)
            .zetels(zetels)
            .kiesdrempel(kiesdrempel)
    }
}
