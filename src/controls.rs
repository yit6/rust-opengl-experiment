pub struct Controls {
    pub space: bool,
    pub shift: bool,
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,

    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
}

impl Controls {
    pub fn new() -> Self {
        Controls {
            space: false,
            shift: false,
            w: false,
            a: false,
            s: false,
            d: false,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

impl Default for Controls {
    fn default() -> Self {
        Controls::new()
    }
}
