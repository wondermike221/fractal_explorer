use crate::coords::Coords;

#[derive(Debug, Copy, Clone)]
pub struct Corners {
    pub first: Option<Coords>,
    pub second: Option<Coords>
}

impl Corners {
    pub fn unpack(self) -> (f64,f64,f64,f64) {
        let ret = (
            self.first.unwrap().x.min(self.second.unwrap().x),
            self.first.unwrap().y.min(self.second.unwrap().y),
            self.first.unwrap().x.max(self.second.unwrap().x),
            self.first.unwrap().y.max(self.second.unwrap().y)
        );
        println!("x: {}, y: {}, x2: {}, y2: {}", ret.0, ret.1, ret.2, ret.3);
        ret
    }
}
