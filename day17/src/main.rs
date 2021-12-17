use std::ops::RangeInclusive;

fn main() {
    let target = (156..=202, -110..=-69);
    dbg!(run(target));
}

struct Probe {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Probe {
    fn new(xvel: i32, yvel: i32) -> Self {
        Self { pos: (0,0), vel: (xvel,yvel) }
    }

    fn step(&mut self) {
        // The probe's x position increases by its x velocity.
        self.pos.0 += self.vel.0;
        // The probe's y position increases by its y velocity.
        self.pos.1 += self.vel.1;
        // Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, 
        //  it decreases by 1 if it is greater than 0,
        //  increases by 1 if it is less than 0, 
        //  or does not change if it is already 0.
        match self.vel.0 {
            x if x > 0 =>  self.vel.0 -= 1,
            x if x < 0 =>  self.vel.0 += 1,
            _ =>  (),
        }
        // Due to gravity, the probe's y velocity decreases by 1.
        self.vel.1 -= 1;
    }
}

fn run(target: (RangeInclusive<i32>, RangeInclusive<i32>)) -> (i32, i32) {
    let mut xvels = Vec::new();
    for xvel in 1.. {
        let mut probe = Probe::new(xvel, 0);
        probe.step();
        if probe.pos.0 > *target.0.end() {
            // if after 1 step we're already past the target, then this is too fast.
            break;
        }
        while probe.pos.0 <= *target.0.end() {
            if target.0.contains(&probe.pos.0) {
                xvels.push(xvel);
                break;
            }
            if probe.vel.0 == 0 {
                break;
            }
            probe.step();
        }
    }

    let mut n_success = 0;
    let mut best_height = 0;
    for yvel in *target.1.start()..=(-*target.1.start()) {
        for xvel in &xvels {
            let mut probe = Probe::new(*xvel, yvel);
            let mut max_height = 0;
            loop {
                max_height = max_height.max(probe.pos.1);
                if target.0.contains(&probe.pos.0) && target.1.contains(&probe.pos.1) {
                    best_height = best_height.max(max_height);
                    n_success += 1;
                    break;
                }
                if probe.pos.1 < *target.1.start() || probe.pos.0 > *target.0.end() {
                    break;
                }
                probe.step();
            }
        }
    }

    (best_height, n_success)
}
