use std::{
    collections::{HashMap, HashSet},
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let mut tr = Track::from_file("tracks.txt")?;
    // let mut tr = Track::from_file("small.txt")?;
    tr.sort_cars();
    // println!("tracks: {:?}", tr);

    loop {
        if let Some((x, y)) = tr.one_step() {
            println!("x,y = {},{}", x, y);
            break;
        }
        // println!("tracks: {:?}", tr);
    }

    // Now continue running (with collisions) until there is only one car
    // left.
    while tr.cars.len() > 1 {
        if let Some((x, y)) = tr.one_step() {
            println!("  Remove at = {},{}", x, y);
        }
    }

    // The final result is the position of the last car.
    println!("x,y = {},{}", tr.cars[0].x, tr.cars[0].y);
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left, Straight, Right,
}

#[derive(Clone, Copy, Debug)]
enum Facing {
    Up, Right, Down, Left,
}

#[derive(Debug)]
struct Car {
    id: usize,   // A unique id for each car, used for removal.
    x: usize,
    y: usize,
    turn: Turn,
    dir: Facing,
}

struct Track {
    track: Vec<Vec<u8>>,
    cars: Vec<Car>,
}

impl fmt::Debug for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cars: HashMap<_, _> = self.cars.iter().map(|car| ((car.x, car.y), car)).collect();

        writeln!(f, "track cars: {:?}", self.cars)?;
        let mut line = String::new();
        for (y, row) in self.track.iter().enumerate() {
            for (x, &b) in row.iter().enumerate() {
                let ch = match cars.get(&(x, y)) {
                    None => b as char,
                    Some(car) => car.dir.char_indicator(),
                };
                line.push(ch);
            }
            writeln!(f, "   {}", line)?;
            line.clear();
        }
        Ok(())
    }
}

impl Track {
    fn from_file<P: AsRef<Path>>(name: P) -> Result<Track> {
        let f = BufReader::new(File::open(name)?);

        let mut track = vec![];
        let mut cars = vec![];

        for (y, line) in f.lines().enumerate() {
            let line = line?;

            // Fix up the cars, replacing the track segments, and
            // separately recording the position of the cars.
            let line = line.bytes().enumerate().map(|(x, ch)| {
                let id = cars.len();
                match ch {
                    b'^' => {
                        cars.push(Car{x: x, y: y, turn: Turn::Left, dir: Facing::Up, id});
                        b'|'
                    }
                    b'v' => {
                        cars.push(Car{x: x, y: y, turn: Turn::Left, dir: Facing::Down, id});
                        b'|'
                    }
                    b'<' => {
                        cars.push(Car{x: x, y: y, turn: Turn::Left, dir: Facing::Left, id});
                        b'-'
                    }
                    b'>' => {
                        cars.push(Car{x: x, y: y, turn: Turn::Left, dir: Facing::Right, id});
                        b'-'
                    }
                    ch => ch,
                }
            }).collect();

            track.push(line);
        }

        Ok(Track{
            track: track,
            cars: cars,
        })
    }

    // Sort the cars, so that the y coordinate is first, then the x.
    fn sort_cars(&mut self) {
        self.cars.sort_by(|a, b| (a.y, a.x).cmp(&(b.y, b.x)));
    }

    /// Take the given Car, and return a new Car adjusted for the movement
    /// (and possible direction change).
    fn move_car(&self, car: &Car) -> Car {
        // Figure out the new position of this particular car.
        let (x, y) = car.dir.step(car.x, car.y);

        let (new_dir, new_turn) = match self.track[y][x] {
            b'/' => (match car.dir {
                Facing::Up => Facing::Right,
                Facing::Right => Facing::Up,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Down,
            }, car.turn),
            b'\\' => (match car.dir {
                Facing::Up => Facing::Left,
                Facing::Left => Facing::Up,
                Facing::Right => Facing::Down,
                Facing::Down => Facing::Right,
            }, car.turn),
            b'+' => (car.dir.apply_turn(car.turn),
                car.turn.next_turn()),
            b'-' | b'|' => (car.dir, car.turn),
            _ => panic!("Moved off of track"),
        };

        Car {
            x, y,
            dir: new_dir,
            turn: new_turn,
            id: car.id,
        }
    }

    /// Apply a single step, returning a collision if there is one, or None
    /// if we didn't find a collision.  Note that when there is a
    /// collision, the two affected cars will be removed.
    fn one_step(&mut self) -> Option<(usize, usize)> {
        let mut places: HashMap<_, _> = self.cars.iter().map(|c| ((c.x, c.y), c.id)).collect();
        self.sort_cars();

        let mut result = None;
        let mut removes: HashSet<usize> = HashSet::new();

        let mut new_cars = Vec::with_capacity(self.cars.len());
        for car in &self.cars {
            if removes.contains(&car.id) {
                continue;
            }
            let new_car = self.move_car(car);
            match places.get(&(new_car.x, new_car.y)) {
                None => (),
                Some(ccar) => {
                    // Return the first colliding coordinate.
                    if result.is_none() {
                        result = Some((new_car.x, new_car.y));
                    }

                    // And mark both cars as being removed.
                    removes.insert(*ccar);
                    removes.insert(new_car.id);

                    // Since both cars are now removed, remove this coord.
                    places.remove(&(new_car.x, new_car.y));
                },
            }

            places.remove(&(car.x, car.y));
            places.insert((new_car.x, new_car.y), new_car.id);

            new_cars.push(new_car);
        }

        // Update the removes, since it is possible for cars to go away
        // because a lower car collides with it.
        self.cars = new_cars.into_iter().filter(|c| !removes.contains(&c.id)).collect();
        result
    }
}

impl Facing {
    fn step(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Facing::Up => (x, y-1),
            Facing::Right => (x+1, y),
            Facing::Down => (x, y+1),
            Facing::Left => (x-1, y),
        }
    }

    fn apply_turn(&self, turn: Turn) -> Facing {
        match turn {
            Turn::Left => match *self {
                Facing::Up => Facing::Left,
                Facing::Right => Facing::Up,
                Facing::Down => Facing::Right,
                Facing::Left => Facing::Down,
            },
            Turn::Straight => *self,
            Turn::Right => match *self {
                Facing::Up => Facing::Right,
                Facing::Right => Facing::Down,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Up,
            }
        }
    }

    fn char_indicator(&self) -> char {
        match *self {
            Facing::Up => '^',
            Facing::Right => '>',
            Facing::Down => 'v',
            Facing::Left => '<',
        }
    }
}

impl Turn {
    fn next_turn(&self) -> Turn {
        match *self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}
