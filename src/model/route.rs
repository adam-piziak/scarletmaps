pub struct Route {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub stops: Vec<RouteStop>
}

pub struct RouteStop {
    pub id: String,
    pub name: String,
    pub arrivals: Vec<f64>
}

impl Route {
    pub fn new(id: String, name: String, stops: Vec<RouteStop>) -> Route {
        Route { id, name, active: false, stops }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn update_arrivals(&mut self, stop_id: String, times: Vec<f64>) {
        self.stops.iter_mut().find(|ref mut x| x.id == stop_id).unwrap().arrivals = times;
    }
}

impl RouteStop {
    pub fn new(id: String, name: String) -> RouteStop {
        RouteStop { id, name, arrivals: Vec::new() }
    }
}
