use std::collections::HashMap;
use super::*;
use player::Player;

pub type Country = &'static str;
pub type TroopCount = usize;

pub struct Ocupation {
    pub player: Player,
    pub troops: TroopCount
}

pub struct Continent {
    name: &'static str,
    countries: Vec<Country>

}

pub struct Map {
    continents: Vec<Continent>,
    ocupations: HashMap<Country, Ocupation>,
    fixed_troops: HashMap<Country, TroopCount>
}

impl Continent {
    pub fn new() -> Self {
        // Guardar lista de paises
        unimplemented!()
    }

    pub fn iter_countries<'a>(self: &Self) -> CountryIterator<'a> {
        unimplemented!()
    }
}

impl Map {
    pub fn new() -> Self {
        // Dada la lista de jugadores
        // Generar ocupaciones iniciales
        // Armar lista de continentes con lista de paises
        unimplemented!()
    }

    pub fn add_troops(self: &mut Self, country: Country, n: TroopCount) {
        unimplemented!()
    }

    pub fn kill_troops(self: &mut Self, country: Country, n: TroopCount) {
        unimplemented!()
    }

    pub fn move_troops(self: &mut Self, from: Country, to: Continent) {
        // mover e incrementar fixed_troops
        unimplemented!()
    }

    pub fn restart_round(self: &mut Self) {
        // restart fixed troops
        unimplemented!()
    }

    pub fn iter_countries<'a>(self: &Self) -> CountryIterator<'a> {
        unimplemented!()
    }

    pub fn iter_continents<'a>(self: &Self) -> ContinentIterator<'a> {
        unimplemented!()
    }

    pub fn get_ocupation(self: &Self, country: Country) -> &Ocupation {
        self.ocupations.get(country).unwrap()
    }
}

pub struct CountryIterator<'a> {
    countries: &'a[Country],
    index: usize
}

impl<'a> Iterator for CountryIterator<'a> {
    type Item = Country;

    fn next(self: &mut Self) -> Option<Self::Item> {
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

    fn next(self: &mut Self) -> Option<Self::Item> {
        let result = if self.index < self.contienents.len() {
            &self.contienents[self.index]
        } else {
            return None
        };
        self.index += 1;
        Some(result)
    }
}