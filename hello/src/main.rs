struct Sensor {
    active: bool,
    latest: u32,
}

impl Sensor {
    fn new() -> Sensor {
        Sensor { active: false, latest: 0 }
    }

    fn init(&mut self) {
        self.active = true;
        self.latest = 100;
    }

    fn print(&self) {
        println!("{}, {}", self.active, self.latest);
    }
}

struct SensorFusion {
    tempr: Sensor,
    light: Sensor,
}

fn inc_sens(s: &mut Sensor) {
    s.latest += 1;
}

struct Image<'a> {
    raw: &'a [u8;256]
}

fn main() {
    let mut sf = SensorFusion{
        tempr: Sensor::new(),
        light: Sensor::new()
    };
    sf.tempr.print();
    sf.light.print();

    inc_sens(&mut sf.tempr);

    sf.tempr.print();
    sf.light.print();

    let image;
    let byte = [0; 256];
    image = Image {
        raw: &byte
    };

    println!("image.raw[0]={}", image.raw[0]);
 }
