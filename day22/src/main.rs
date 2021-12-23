fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}

struct Cube {
    value: bool,
    bounds: ((i32, i32), (i32, i32), (i32, i32)),
}

enum CubeCompare {
    Contains,
    Split(Split),
    Disjoint,
}

enum Split {
    X(i32),
    Y(i32),
    Z(i32),
}

impl Cube {
    fn compare(&self, voxel: &Voxel) -> CubeCompare {
        if (self.bounds.0 .0 <= voxel.bounds.0 .0 && voxel.bounds.0 .1 <= self.bounds.0 .1)
            && (self.bounds.1 .0 <= voxel.bounds.1 .0 && voxel.bounds.1 .1 <= self.bounds.1 .1)
            && (self.bounds.2 .0 <= voxel.bounds.2 .0 && voxel.bounds.2 .1 <= self.bounds.2 .1)
        {
            CubeCompare::Contains
        } else if (voxel.bounds.0 .1 < self.bounds.0 .0 || self.bounds.0 .1 < voxel.bounds.0 .0)
            || (voxel.bounds.1 .1 < self.bounds.1 .0 || self.bounds.1 .1 < voxel.bounds.1 .0)
            || (voxel.bounds.2 .1 < self.bounds.2 .0 || self.bounds.2 .1 < voxel.bounds.2 .0)
        {
            CubeCompare::Disjoint
        } else {
            let intersect = |a: (i32, i32), b: (i32, i32)| ((a.0.max(b.0)), a.1.min(b.1));
            let sub = (
                intersect(self.bounds.0, voxel.bounds.0),
                intersect(self.bounds.1, voxel.bounds.1),
                intersect(self.bounds.2, voxel.bounds.2),
            );
            let split = if sub.0 .0 != voxel.bounds.0 .0 {
                Split::X(sub.0 .0)
            } else if sub.0 .1 != voxel.bounds.0 .1 {
                Split::X(sub.0 .1 + 1)
            } else if sub.1 .0 != voxel.bounds.1 .0 {
                Split::Y(sub.1 .0)
            } else if sub.1 .1 != voxel.bounds.1 .1 {
                Split::Y(sub.1 .1 + 1)
            } else if sub.2 .0 != voxel.bounds.2 .0 {
                Split::Z(sub.2 .0)
            } else if sub.2 .1 != voxel.bounds.2 .1 {
                Split::Z(sub.2 .1 + 1)
            } else {
                unimplemented!();
            };
            CubeCompare::Split(split)
        }
    }
}

fn parse(input: &str) -> Vec<Cube> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (onoff, coords) = line.trim().split_once(" ").unwrap();
            let mut bounds = coords.split(",").map(|c| {
                let strs = c.split_once("=").unwrap().1.split_once("..").unwrap();
                (strs.0.parse().unwrap(), strs.1.parse().unwrap())
            });
            Cube {
                value: onoff == "on",
                bounds: (
                    bounds.next().unwrap(),
                    bounds.next().unwrap(),
                    bounds.next().unwrap(),
                ),
            }
        })
        .collect()
}

enum VoxelState {
    Whole(bool),
    Split(Box<[Voxel; 2]>),
}

struct Voxel {
    bounds: ((i32, i32), (i32, i32), (i32, i32)),
    state: VoxelState,
}

impl Voxel {
    fn set(&mut self, cube: &Cube) {
        match &mut self.state {
            VoxelState::Whole(val) => {
                if *val != cube.value {
                    match cube.compare(self) {
                        CubeCompare::Contains => self.state = VoxelState::Whole(cube.value),
                        CubeCompare::Split(s) => {
                            let mut subvoxels = self.split_at(&s);
                            for vox in subvoxels.iter_mut() {
                                vox.set(cube);
                            }
                            self.state = VoxelState::Split(Box::new(subvoxels));
                        }
                        CubeCompare::Disjoint => (),
                    }
                }
            }

            VoxelState::Split(subvoxels) => {
                for vox in subvoxels.iter_mut() {
                    vox.set(cube);
                }
            }
        }
    }

    fn split_at(&self, s: &Split) -> [Voxel; 2] {
        let whole_value = if let VoxelState::Whole(value) = &self.state {
            *value
        } else {
            unimplemented!();
        };
        let mut result = [
            Voxel {
                bounds: self.bounds,
                state: VoxelState::Whole(whole_value),
            },
            Voxel {
                bounds: self.bounds,
                state: VoxelState::Whole(whole_value),
            },
        ];
        match s {
            Split::X(x) => {
                result[0].bounds.0 .1 = x - 1;
                result[1].bounds.0 .0 = *x;
            }
            Split::Y(y) => {
                result[0].bounds.1 .1 = y - 1;
                result[1].bounds.1 .0 = *y;
            }
            Split::Z(z) => {
                result[0].bounds.2 .1 = z - 1;
                result[1].bounds.2 .0 = *z;
            }
        }
        result
    }

    fn volume(&self) -> usize {
        (self.bounds.0 .1 - self.bounds.0 .0 + 1) as usize
            * (self.bounds.1 .1 - self.bounds.1 .0 + 1) as usize
            * (self.bounds.2 .1 - self.bounds.2 .0 + 1) as usize
    }

    fn count(&self) -> usize {
        match &self.state {
            VoxelState::Whole(value) => {
                if *value {
                    self.volume()
                } else {
                    0
                }
            }
            VoxelState::Split(subs) => subs.iter().map(|v| v.count()).sum(),
        }
    }
}

fn part1(input: &str) -> usize {
    let cubes = parse(input);
    let mut reactor = Voxel {
        bounds: ((-50, 50), (-50, 50), (-50, 50)),
        state: VoxelState::Whole(false),
    };
    for cube in cubes {
        reactor.set(&cube);
    }
    reactor.count()
}

fn part2(input: &str) -> usize {
    let cubes = parse(input);
    let mut bounds = (
        (i32::MIN, i32::MAX),
        (i32::MIN, i32::MAX),
        (i32::MIN, i32::MAX),
    );
    bounds.0 .0 = cubes.iter().map(|c| c.bounds.0 .0).min().unwrap();
    bounds.1 .0 = cubes.iter().map(|c| c.bounds.1 .0).min().unwrap();
    bounds.2 .0 = cubes.iter().map(|c| c.bounds.2 .0).min().unwrap();
    bounds.0 .1 = cubes.iter().map(|c| c.bounds.0 .1).max().unwrap();
    bounds.1 .1 = cubes.iter().map(|c| c.bounds.1 .1).max().unwrap();
    bounds.2 .1 = cubes.iter().map(|c| c.bounds.2 .1).max().unwrap();
    let mut reactor = Voxel {
        bounds,
        state: VoxelState::Whole(false),
    };
    for cube in cubes {
        reactor.set(&cube);
    }
    reactor.count()
}

#[test]
fn test_part1() {
    let input = "on x=-20..26,y=-36..17,z=-47..7
    on x=-20..33,y=-21..23,z=-26..28
    on x=-22..28,y=-29..23,z=-38..16
    on x=-46..7,y=-6..46,z=-50..-1
    on x=-49..1,y=-3..46,z=-24..28
    on x=2..47,y=-22..22,z=-23..27
    on x=-27..23,y=-28..26,z=-21..29
    on x=-39..5,y=-6..47,z=-3..44
    on x=-30..21,y=-8..43,z=-13..34
    on x=-22..26,y=-27..20,z=-29..19
    off x=-48..-32,y=26..41,z=-47..-37
    on x=-12..35,y=6..50,z=-50..-2
    off x=-48..-32,y=-32..-16,z=-15..-5
    on x=-18..26,y=-33..15,z=-7..46
    off x=-40..-22,y=-38..-28,z=23..41
    on x=-16..35,y=-41..10,z=-47..6
    off x=-32..-23,y=11..30,z=-14..3
    on x=-49..-5,y=-3..45,z=-29..18
    off x=18..30,y=-20..-8,z=-3..13
    on x=-41..9,y=-7..43,z=-33..15
    on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
    on x=967..23432,y=45373..81175,z=27513..53682";
    assert_eq!(part1(input), 590784);
}
