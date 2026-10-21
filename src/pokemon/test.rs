use crate::pokemon::{Pokemon, PrimitivePokemon, nature::Nature, stat::{EffortValues, Stats}};


#[test]
fn assert_stats() {
    let garchomp = PrimitivePokemon {
        species: "Garchomp".to_string(),
        base_stats: Stats::new(108, 130, 95, 80, 85, 102),
        types: heapless::Vec::new(),
    };

    let evs = EffortValues::new(0, 252, 0, 4, 0, 252).unwrap();

    Pokemon::from_primitive(garchomp).with_evs(evs).with_nature(Nature::Adamant).with_level(74);
}