use super::*;

pub struct ZetelVerdelerBuilder<K: Debug + Clone + PartialEq + Eq + Hash> {
    stem_aantallen: HashMap<K, u32>,
    kiesdrempel_perc: Option<f32>,
    zetels: Option<u32>,
}

impl<K: Debug + Clone + PartialEq + Eq + Hash> ZetelVerdelerBuilder<K> {
    pub fn new(stem_aantallen: &[(K, u32)]) -> Self {
        Self {
            stem_aantallen: stem_aantallen.iter().cloned().collect(),
            kiesdrempel_perc: None,
            zetels: None,
        }
    }

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
