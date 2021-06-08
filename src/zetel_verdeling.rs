use super::*;

#[derive(Debug)]
pub struct ZetelVerdeling<T, U> {
    zetels: HashMap<(T, U), u32>,
    totalen: HashMap<U, u32>,
    partijen: HashSet<U>,
    kieskringen: HashSet<T>,
}

impl<T, U> ZetelVerdeling<T, U> {
    pub fn new() -> Self {
        Self {
            zetels: HashMap::new(),
            totalen: HashMap::new(),
            partijen: HashSet::new(),
            kieskringen: HashSet::new(),
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn raw(
        self,
    ) -> (
        HashMap<(T, U), u32>,
        HashMap<U, u32>,
        HashSet<T>,
        HashSet<U>,
    ) {
        (self.zetels, self.totalen, self.kieskringen, self.partijen)
    }

    pub fn iter(&self) -> impl Iterator<Item = ((&T, &U), u32)> {
        self.zetels.iter().map(|((t, u), z)| ((t, u), *z))
    }

    pub fn iter_flat(&self) -> impl Iterator<Item = (&T, &U, u32)> {
        self.zetels.iter().map(|((t, u), z)| (t, u, *z))
    }
}

impl<T, U> Default for zetel_verdeling::ZetelVerdeling<T, U> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq + Hash, U: Eq + Hash> ZetelVerdeling<T, U> {
    pub fn get(&self, kieskring: T, partij: U) -> Option<u32> {
        self.zetels.get(&(kieskring, partij)).copied()
    }

    pub fn totaal(&self, partij: &U) -> Option<u32> {
        self.totalen.get(partij).copied()
    }
}

impl<T: Clone + Eq + Hash, U: Clone + Eq + Hash> ZetelVerdeling<T, U> {
    pub fn partijen(&self) -> impl Iterator<Item = U> + '_ {
        self.partijen.iter().cloned()
    }

    pub fn kieskringen(&self) -> impl Iterator<Item = T> + '_ {
        self.kieskringen.iter().cloned()
    }
}

impl<T: Clone + Eq + Hash, U: Clone + Eq + Hash> FromIterator<((T, U), u32)>
    for ZetelVerdeling<T, U>
{
    fn from_iter<I: IntoIterator<Item = ((T, U), u32)>>(iter: I) -> Self {
        iter.into_iter().fold(
            ZetelVerdeling::new(),
            |mut acc, ((kieskring, partij), zetel_aantal)| {
                acc.zetels
                    .insert((kieskring.clone(), partij.clone()), zetel_aantal);
                *acc.totalen.entry(partij.clone()).or_insert(0) += zetel_aantal;
                acc.partijen.insert(partij);
                acc.kieskringen.insert(kieskring);
                acc
            },
        )
    }
}

impl<T: Clone + Eq + Hash, U: Clone + Eq + Hash> FromIterator<(T, U, u32)>
    for ZetelVerdeling<T, U>
{
    fn from_iter<I: IntoIterator<Item = (T, U, u32)>>(iter: I) -> Self {
        iter.into_iter().fold(
            ZetelVerdeling::new(),
            |mut acc, (kieskring, partij, zetel_aantal)| {
                acc.zetels
                    .insert((kieskring.clone(), partij.clone()), zetel_aantal);
                *acc.totalen.entry(partij.clone()).or_insert(0) += zetel_aantal;
                acc.partijen.insert(partij);
                acc.kieskringen.insert(kieskring);
                acc
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect() {
        let mut raw_h = HashMap::new();
        raw_h.insert(("zonder drempel", "Partij A"), 13);
        raw_h.insert(("zonder drempel", "Partij B"), 8);
        raw_h.insert(("zonder drempel", "Partij C"), 4);
        raw_h.insert(("met drempel", "Partij A"), 16);
        raw_h.insert(("met drempel", "Partij B"), 9);
        raw_h.insert(("met drempel", "Partij C"), 0);

        let zv = raw_h.into_iter().collect::<ZetelVerdeling<_, _>>();
        assert_eq!(13, zv.get("zonder drempel", "Partij A").unwrap());
        assert_eq!(8, zv.get("zonder drempel", "Partij B").unwrap());
        assert_eq!(4, zv.get("zonder drempel", "Partij C").unwrap());
        assert_eq!(16, zv.get("met drempel", "Partij A").unwrap());
        assert_eq!(9, zv.get("met drempel", "Partij B").unwrap());
        assert_eq!(0, zv.get("met drempel", "Partij C").unwrap());
    }
}
