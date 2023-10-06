use std::collections::HashMap;

pub struct Route {
    pub calls: HashMap<String, Box<dyn Fn()>>,
}

impl Route {}
