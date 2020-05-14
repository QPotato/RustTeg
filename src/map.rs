
use std::collections::HashMap;
use super::*;
use player::Player;
use serde::{Serialize};

pub type Country = &'static str;
pub type TroopCount = usize;

#[derive(Debug, Serialize)]
pub struct Ocupation {
    pub player: Player,
    pub troops: TroopCount
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize)]
pub struct Continent {
    pub name: &'static str,
    countries: Vec<Country>

}

#[derive(Debug, Serialize)]
pub struct Map {
    continents: Vec<Continent>,
    ocupations: HashMap<Country, Ocupation>,
    fixed_troops: HashMap<Country, TroopCount>,
    borders: HashMap<Country, Vec<Country>>,
}

impl Continent {
    pub fn new(name: &'static str, countries: Vec<Country>) -> Self {
        // Guardar lista de paises
        Self {
            name,
            countries
        }
    }

    pub fn iter_countries<'a>(&self) -> CountryIterator<'a> {
        unimplemented!()
    }
}

impl Map {
    pub fn new() -> Self {
        // Dada la lista de jugadores
        // Generar ocupaciones iniciales
        // Armar lista de continentes con lista de paises
        Self {
            continents: vec![
                Continent::new("Europa", vec!["Polonia", "Alemania"])
            ],
            ocupations: vec![
                ("Alemania", Ocupation {
                    player: Player::Black,
                    troops: 1,
                }),
                ("Polonia", Ocupation {
                    player: Player::Red,
                    troops: 4,
                })
            ].into_iter().collect(),
            fixed_troops: vec![].into_iter().collect(),
            borders: vec![
                ("Alemania", vec!["Francia"]),
                ("Francia", vec!["Alemania"]),
            ].into_iter().collect(),
        }
    }

    pub fn add_troops(&mut self, country: Country, n: TroopCount) {
        unimplemented!()
    }

    pub fn kill_troops(&mut self, country: Country, n: TroopCount) {
        unimplemented!()
    }

    pub fn move_troops(&mut self, from: Country, to: Continent) {
        // mover e incrementar fixed_troops
        unimplemented!()
    }

    pub fn restart_round(&mut self) {
        // restart fixed troops
        unimplemented!()
    }

    pub fn iter_countries<'a>(& self) -> CountryIterator<'a> {
        unimplemented!()
    }

    pub fn iter_continents<'a>(&self) -> ContinentIterator<'a> {
        unimplemented!()
    }

    pub fn get_ocupation(&self, country: Country) -> &Ocupation {
        self.ocupations.get(country).unwrap()
    }

    pub fn count_countries(&self, player: Player) -> usize {
        self.iter_countries()
            .filter(|c| self.get_ocupation(c).player == player)
            .count()
    }

    pub fn count_countries_on_continent(&self, player: Player, continent_name: &str) -> usize {
        self.iter_continents()
            .find(|con| con.name == continent_name).unwrap()
            .iter_countries()
            .filter(|c| self.get_ocupation(c).player == player)
            .count()
    }

    pub fn get_continent_size(&self, continent_name: &str) -> usize {
        self.iter_continents()
            .find(|con| con.name == continent_name).unwrap()
            .iter_countries()
            .count()
    }

    pub fn contient_is_ocupied_by(&self, continent_name: &str, player: Player) -> bool {
        self.iter_continents()
            .find(|con| con.name == continent_name).unwrap()
            .iter_countries()
            .all(|c| self.get_ocupation(c).player == player)
    }

    pub fn is_in_continent(&self, country: Country, continent_name: &str) -> bool {
        self.iter_continents()
            .find(|con| con.name == continent_name).unwrap()
            .iter_countries()
            .any(|c| c == country)
    }

    pub fn get_borders(&self, a: Country) -> Vec<Country> {
        self.borders.get(a).unwrap().clone()
    }

    pub fn adjacent(&self, a: Country, b: Country) -> bool {
        self.borders.get(a).unwrap().iter().find(|c| **c == b).is_some()
    }

    pub fn get_posible_attacks(&self, player: Player) -> HashMap<Country, Vec<Country>> {
        self.iter_countries()
            .filter(|c| self.get_ocupation(c).player == player)
            .map(|c| (c, self.get_borders(c)))
            .collect()
    }
}

#[derive(Clone)]
pub struct CountryIterator<'a> {
    countries: &'a[Country],
    index: usize
}

impl<'a> Iterator for CountryIterator<'a> {
    type Item = Country;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.index < self.countries.len() {
            &self.countries[self.index]
        } else {
            return None
        };
        self.index += 1;
        Some(result)
    }
}

pub struct ContinentIterator<'a> {
    contienents: &'a[Continent],
    index: usize
}

impl<'a> Iterator for ContinentIterator<'a> {
    type Item = &'a Continent;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.index < self.contienents.len() {
            &self.contienents[self.index]
        } else {
            return None
        };
        self.index += 1;
        Some(result)
    }
}