fn main() {
    // println!("{}", Rack::new(8).level(3, 5));
    // println!("{}", Rack::new(57).level(122, 79));
    // println!("{}", Rack::new(39).level(217, 196));
    // println!("{}", Rack::new(71).level(101, 153));
    // println!("{}", Rack::new(18).power_grid(33, 45));
    // println!("{}", Rack::new(42).power_grid(21, 61));
    let r = Rack::new(7347);

    let mut biggest = std::i32::MIN;
    let mut best = (0, 0);
    for y in 1 .. 299 {
        for x in 1 .. 299 {
            let tmp = r.power_grid(x, y, 3);
            if tmp > biggest {
                biggest = tmp;
                best = (x, y);
            }
        }
    }
    println!("result1: {:?}", best);

    // This is pretty untenable, so we need to come up with a better way.
    let mut biggest = std::i32::MIN;
    let mut best = (0, 0, 0);
    for size in 1 ..= 300 {
        println!("size: {}", size);
        for y in 1 ..= 300 - size + 1 {
            for x in 1 ..= 300 - size + 1 {
                let tmp = r.power_grid(x, y, size);
                if tmp > biggest {
                    biggest = tmp;
                    best = (x, y, size);
                }
            }
        }
    }
    println!("{:?}", best);
}

struct Rack {
    serial: i32,

    /// Indexed by (y-1)*300+(x-1).
    levels: Vec<i32>,
}

impl Rack {
    fn new(serial: i32) -> Rack {
        let mut levels = vec![0i32; 300*300];
        for y in 0 .. 300 {
            for x in 0 .. 300 {
                levels[(y*300+x) as usize] = (((x + 11) * (y + 1) + serial) * (x + 11) / 100) % 10 - 5;
            }
        }
        Rack {
            serial: serial,
            levels: levels,
        }
    }

    fn level(&self, x: i32, y: i32) -> i32 {
        // (((x + 10) * y + self.serial) * (x + 10) / 100) % 10 - 5
        self.levels[((y-1)*300+x-1) as usize]
    }

    fn power_grid(&self, x: i32, y: i32, size: i32) -> i32 {
        (y .. y + size).map(|y| {
            (x .. x + size).map(|x| self.level(x, y)).sum::<i32>()
        }).sum()
    }
}
