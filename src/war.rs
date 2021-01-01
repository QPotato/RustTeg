use super::*;
use player::*;
use map::*;

pub type AttackResult = (usize, usize);


pub fn attack(map: &Map, attacker: Country, defender: Country) -> AttackResult {
    // TODO
    let o1 = map.get_ocupation(attacker);
    let o2 = map.get_ocupation(defender);

    assert!(o1.troops > 1);
    assert!(o1.player != o2.player);

    // let roll_size

    // TODO: dice rolls
    let d1 = vec![1, 1, 1];
    let d2 = vec![1, 1, 1];


    (3, 0)
}

