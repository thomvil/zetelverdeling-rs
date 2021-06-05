use std::{collections::HashMap, hash::Hash, ops::RangeInclusive, fmt::Debug};

mod kiesdelersystemen;
mod zetel_verdeler;
mod zetel_verdeler_builder;

pub use zetel_verdeler::*;
pub use zetel_verdeler_builder::*;

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
