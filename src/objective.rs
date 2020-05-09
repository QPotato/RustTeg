use std::collections::HashMap;
use super::*;
use map::*;


pub struct Objective {
    description: String,
    test: Box<dyn Fn(Map) -> bool>,
}

impl Objective {
    pub fn new() -> Self {
        // Guardar lista de paises
        unimplemented!()
    }
}