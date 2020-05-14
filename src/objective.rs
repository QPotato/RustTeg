extern crate rand;
use std::fmt;
use super::*;
use map::*;

type ObjectiveTest = fn(Player, &Map) -> bool;

pub struct Objective {
    description: &'static str,
    test: ObjectiveTest,
}

pub struct ObjectivePool {
    pool: Vec<Objective>
}

impl Objective {
    pub fn new() -> Self {
        // Guardar lista de paises
        Objective {
            description: "Objetivo comun",
            test: |player, map| {
                map.count_countries(player) >= 30
            },
        }
    }

    pub fn is_completed(&self, player: Player, map: &Map) -> bool {
        (self.test)(player, map)
    }
}

impl ObjectivePool {
    pub fn new() -> Self {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let mut rng = thread_rng();
        let mut pool = vec![
            Objective {
                description: "Ocupar África, 5 países de América del Norte y 4 países de Europa.",
                test: |player, map| {
                    map.contient_is_ocupied_by("África", player)
                    && map.count_countries_on_continent(player, "América del Norte") >= 5
                    && map.count_countries_on_continent(player, "Europa") >= 4
                },
            },
            Objective {
                description: "Ocupar América del Sur, 7 países de Europa y 3 países limítrofes entre sí en cualquier lugar del mapa.",
                test: |player, map| {
                    use itertools::izip;
                    map.contient_is_ocupied_by("América del Sur", player)
                    && map.count_countries_on_continent(player, "Europa") >= 7
                    && {
                        let candidates = map.iter_countries()
                            .filter(|c| map.get_ocupation(c).player == player)
                            .filter(|c| !map.is_in_continent(c, "Europa") && !map.is_in_continent(c, "América del Sur"));
                        izip!(candidates.clone(), candidates.clone(), candidates)
                            .any(|(a, b, c)| map.adjacent(a, b) && map.adjacent(b, c) && map.adjacent(c, a))
                    }
                },
            },
            Objective {
                description: "Ocupar Asia y 2 países de América del Sur.",
                test: |player, map| {
                    map.contient_is_ocupied_by("Asia", player)
                    && map.count_countries_on_continent(player, "América del Sur") >= 2
                },
            },
            Objective {
                description: "Ocupar América del Norte, 2 países de Oceanía y 4 de Asia.",
                test: |player, map| {
                    map.contient_is_ocupied_by("Europa", player)
                    && map.count_countries_on_continent(player, "Asia") >= 4
                    && map.count_countries_on_continent(player, "América del Sur") >= 2
                },
            },
            Objective {
                description: "Ocupar 2 países de Oceanía, 2 países de África, 2 países de América del Sur, 3 países de Europa, 4 de América del Norte y 3 de Asia.",
                test: |player, map| {
                    map.count_countries_on_continent(player, "Oceanía") >= 2
                    && map.count_countries_on_continent(player, "África") >= 2
                    && map.count_countries_on_continent(player, "América del Sur") >= 2
                    && map.count_countries_on_continent(player, "Europa") >= 2
                    && map.count_countries_on_continent(player, "América del Norte") >= 4
                    && map.count_countries_on_continent(player, "Asia") >= 3
                },
            },
            Objective {
                description: "Ocupar Oceanía, América del Norte y 2 países de Europa.",
                test: |player, map| {
                    map.contient_is_ocupied_by("Oceanía", player)
                    && map.contient_is_ocupied_by("América del Norte", player)
                    && map.count_countries_on_continent(player, "Europa") >= 2
                },
            },
            Objective {
                description: "Ocupar América del Sur, África y 5 países de América del Norte.",
                test: |player, map| {
                    map.contient_is_ocupied_by("África", player)
                    && map.contient_is_ocupied_by("América del Sur", player)
                    && map.count_countries_on_continent(player, "América del Norte") >= 5
                },
            },
        ];
        pool.shuffle(&mut rng);
        ObjectivePool { pool }
    }

    pub fn get(&mut self) -> Objective {
        self.pool.remove(0)
    }
}

impl fmt::Debug for Objective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.description.fmt(f)
    }
}