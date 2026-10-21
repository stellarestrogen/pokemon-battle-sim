use crate::pokemon::{
    Pokemon, PrimitivePokemon,
    nature::Nature,
    stat::{EffortValues, IndividualValues, StatType, Stats},
};

#[test]
fn assert_stats() {
    let garchomp = PrimitivePokemon {
        species: "Garchomp".to_string(),
        base_stats: Stats::new(108, 130, 95, 80, 85, 102),
        types: heapless::Vec::new(),
    };

    let ivs = IndividualValues::new(24, 12, 30, 16, 23, 5).unwrap();
    let evs = EffortValues::new(74, 190, 91, 48, 84, 23).unwrap();

    let garchomp = Pokemon::from_primitive(garchomp)
        .with_ivs(ivs)
        .with_evs(evs)
        .with_nature(Nature::Adamant)
        .with_level(78)
        .build();

    assert_eq!(289, garchomp.get_stat(StatType::HP));
    assert_eq!(278, garchomp.get_stat(StatType::Attack));
    assert_eq!(193, garchomp.get_stat(StatType::Defense));
    assert_eq!(135, garchomp.get_stat(StatType::SpecialAttack));
    assert_eq!(171, garchomp.get_stat(StatType::SpecialDefense));
    assert_eq!(171, garchomp.get_stat(StatType::Speed));
}
