use std::fmt::Debug;
use std::io;
use std::str::FromStr;

fn get_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

fn get_value<T>() -> T
where
    T: std::str::FromStr,
    <T as FromStr>::Err: Debug,
{
    let input_line = get_line();
    input_line.trim().parse::<T>().unwrap()
}

#[derive(Debug, Default, Copy, Clone)]
struct Vec2 {
    x: i64,
    y: i64,
}
impl Vec2 {
    fn distance(&self, pos: Vec2) -> i64 {
        let dx = self.x - pos.x;
        let dy = self.y - pos.y;
        dx * dx + dy * dy
    }
}

struct Drone {
    _id: i32,
    pos: Vec2,
    _emergency: i32,
    battery: i32,
}

impl Drone {
    fn from_input() -> Self {
        let input_line = get_line();
        let mut inputs = input_line.split_whitespace();
        let drone_id = inputs.next().unwrap().parse().unwrap();
        let drone_x = inputs.next().unwrap().parse().unwrap();
        let drone_y = inputs.next().unwrap().parse().unwrap();
        let emergency = inputs.next().unwrap().parse().unwrap();
        let battery = inputs.next().unwrap().parse().unwrap();
        Drone {
            _id: drone_id,
            pos: Vec2 {
                x: drone_x,
                y: drone_y,
            },
            _emergency: emergency,
            battery,
        }
    }
}

struct Drones {
    drones: Vec<Drone>,
}

impl Drones {
    fn new() -> Self {
        Drones {
            drones: Vec::new(),
        }
    }

    fn update_from_input(&mut self) {
        self.drones.clear();
        let drone_count: usize = get_value();
        self.drones.reserve_exact(drone_count);
        for _ in 0..drone_count {
            self.drones.push(Drone::from_input());
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Creature {
    id: usize,
    pos: Vec2,
    speed: Vec2,
    _color: u8,
    _type: u8,
    scanned: bool,
}

impl Creature {
    fn from_input() -> Self {
        let input_line = get_line();
        let mut inputs = input_line.split_whitespace();
        let creature_id = inputs.next().unwrap().parse().unwrap();
        let color = inputs.next().unwrap().parse().unwrap();
        let _type = inputs.next().unwrap().parse().unwrap();
        Creature {
            id: creature_id,
            _color: color,
            _type,
            scanned: false,
            ..Default::default()
        }
    }

    fn set_scanned(&mut self) {
        self.scanned = true;
    }
}

#[derive(Debug)]
struct Creatures {
    creatures: Vec<Creature>,
}

impl Creatures {
    fn get_mut(&mut self, id: usize) -> &mut Creature {
        let idx = id - self.creatures[0].id;
        let res = &mut self.creatures[idx];
        assert_eq!(res.id, id);
        res
    }

    fn from_input() -> Self {
        let creature_count: usize = get_value();
        let creatures = (0..creature_count)
            .map(|_| Creature::from_input())
            .collect::<Vec<_>>();
        // creatures.sort_by_key(|c| c.id); // let's assume they are already sorted
        assert!(creatures.windows(2).all(|w| w[0].id + 1 == w[1].id));
        Creatures { creatures }
    }

    fn update_creature_from_input(&mut self) {
        let input_line = get_line();
        let mut inputs = input_line.split_whitespace();
        let creature_id = inputs.next().unwrap().parse().unwrap();
        let creature_x = inputs.next().unwrap().parse().unwrap();
        let creature_y = inputs.next().unwrap().parse().unwrap();
        let creature_vx = inputs.next().unwrap().parse().unwrap();
        let creature_vy = inputs.next().unwrap().parse().unwrap();
        let creature = self.get_mut(creature_id);
        creature.pos.x = creature_x;
        creature.pos.y = creature_y;
        creature.speed.x = creature_vx;
        creature.speed.y = creature_vy;
    }

    fn update_from_input(&mut self) {
        let creature_count: usize = get_value();
        for _ in 0..creature_count {
            self.update_creature_from_input();
        }
    }

    fn find_target(&self, pos: Vec2) -> Option<Vec2> {
        self.creatures
            .iter()
            .filter(|c| !c.scanned)
            .min_by_key(|c| c.pos.distance(pos))
            .map(|c| c.pos)
    }

    fn update_scanned_from_input(&mut self) {
        let scan_count: usize = get_value();
        for _ in 0..scan_count {
            let creature_id: usize = get_value();
            self.get_mut(creature_id).set_scanned();
            eprint!(" {}", creature_id);
        }
        eprintln!();
    }
}

/**
 * Score points by scanning valuable fish faster than your opponent.
 **/
fn main() {
    let mut creatures = Creatures::from_input();
    let mut my_drones = Drones::new();
    let mut foes_drones = Drones::new();

    eprintln!("creatures: {:?}", creatures.creatures);

    // game loop
    loop {
        let my_score: i32 = get_value();
        let foe_score: i32 = get_value();
        eprintln!("{} - {}", my_score, foe_score);
        eprint!("my scan:");
        creatures.update_scanned_from_input();
        eprint!("foe scan:");
        let foe_scan_count: usize = get_value();
        for _ in 0..foe_scan_count {
            let creature_id: i32 = get_value();
            eprint!(" {}", creature_id);
        }
        eprintln!();
        my_drones.update_from_input();
        foes_drones.update_from_input();
        let drone_scan_count: usize = get_value();
        for _ in 0..drone_scan_count {
            let input_line = get_line();
            let mut inputs = input_line.split_whitespace();
            let drone_id: i32 = inputs.next().unwrap().parse().unwrap();
            let creature_id: i32 = inputs.next().unwrap().parse().unwrap();
            eprintln!("drone {} scanned creature {}", drone_id, creature_id);
        }
        creatures.update_from_input();
        let radar_blip_count: usize = get_value();
        for _ in 0..radar_blip_count {
            let input_line = get_line();
            let mut inputs = input_line.split_whitespace();
            let drone_id: i32 = inputs.next().unwrap().parse().unwrap();
            let creature_id: i32 = inputs.next().unwrap().parse().unwrap();
            let radar = inputs.next().unwrap().trim().to_string();
            eprintln!(
                "drone {} radar blip {} at {}",
                drone_id, creature_id, radar
            );
        }
        for drone in &my_drones.drones {
            let Some(target_pos) = creatures.find_target(drone.pos) else {
                println!("WAIT 0"); // MOVE <x> <y> <light (1|0)> | WAIT <light (1|0)>
                continue;
            };
            print!("MOVE {} {}", target_pos.x, target_pos.y);
            if drone.battery > 15 {
                print!(" 1");
            } else {
                print!(" 0");
            }
            println!(" eheh");
        }
    }
}
