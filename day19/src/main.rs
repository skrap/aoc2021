use std::collections::HashSet;

fn main() {
    let input = include_str!("input");
    dbg!(run(input));
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct Pos(i32, i32, i32);

impl Pos {
    fn transl(&self, b: &Pos) -> Pos {
        Pos(self.0 + b.0, self.1 + b.1, self.2 + b.2)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Scanner {
    beacons: Vec<Pos>,
    pos: Pos,
    id: usize,
}

impl Scanner {
    /// roll clockwise along x axis
    fn roll_x(&mut self) {
        assert_eq!(self.pos, Pos(0,0,0));
        for beacon in self.beacons.iter_mut() {
            let old = *beacon;
            beacon.1 = old.2;
            beacon.2 = -old.1;
        }
    }

    /// pitch on z axis
    fn pitch_z(&mut self) {
        assert_eq!(self.pos, Pos(0,0,0));
        for beacon in self.beacons.iter_mut() {
            let old = *beacon;
            beacon.0 = old.1;
            beacon.1 = -old.0;
        }
    }

    /// yaw on y axis
    fn yaw_y(&mut self) {
        assert_eq!(self.pos, Pos(0,0,0));
        for beacon in self.beacons.iter_mut() {
            let old = *beacon;
            beacon.0 = -old.2;
            beacon.2 = old.0;
        }
    }

    fn can_see(&self, pos: &Pos) -> bool {
        (self.pos.0 - pos.0).abs() <= 1000
            && (self.pos.1 - pos.1).abs() <= 1000
            && (self.pos.2 - pos.2).abs() <= 1000
    }

    fn transl(&self, offset: &Pos) -> Self {
        Scanner {
            beacons: self.beacons.iter().map(|pos| pos.transl(&offset)).collect(),
            pos: self.pos.transl(&offset),
            id: self.id,
        }
    }
    
    fn rolls(mut self) -> Vec<Self> {
        let mut result = Vec::new();
        result.push(self.clone());
        self.roll_x();
        result.push(self.clone());
        self.roll_x();
        result.push(self.clone());
        self.roll_x();
        result.push(self);
        result
    }

    fn orientations(&self) -> Vec<Self> {
        let mut result = self.clone().rolls();
        let mut pig = self.clone();

        pig.yaw_y();
        result.append(&mut pig.clone().rolls());
        
        pig.yaw_y();
        result.append(&mut pig.clone().rolls());
        
        pig.yaw_y();
        result.append(&mut pig.clone().rolls());

        pig.pitch_z();
        result.append(&mut pig.clone().rolls());

        pig.pitch_z();
        pig.pitch_z();
        result.append(&mut pig.rolls());

        assert_eq!(result.len(), 24);

        result
    }
}

fn run(input: &str) -> (usize,i32) {
    let mut scanners = parse(input);
    let mut solved = vec![scanners[0].clone()];
    scanners.remove(0);
    'next: while !scanners.is_empty() {
        for (s2_idx, s2) in scanners.iter().enumerate() {
            for s1 in &solved {
                if let Some(s2) = do_overlap(s1, s2) {
                    scanners.remove(s2_idx);
                    solved.push(s2);
                    continue 'next;
                }
            }
        }
        let all_beacons: HashSet<_> = solved.iter().map(|s| s.beacons.iter()).flatten().collect();
        let mut all_beacons: Vec<_> = all_beacons.into_iter().collect();
        all_beacons.sort();
        for beacon in all_beacons {
            println!("{},{},{}", beacon.0, beacon.1, beacon.2);
        }
        unimplemented!();
    }

    let all_beacons: HashSet<_> = solved.iter().map(|s| s.beacons.iter()).flatten().collect();

    let max_dist = solved.iter().map(|s1| {
        solved.iter().map(|s2| {
            (s1.pos.0 - s2.pos.0).abs() + 
            (s1.pos.1 - s2.pos.1).abs() + 
            (s1.pos.2 - s2.pos.2).abs()
        }).max().unwrap()
    }).max().unwrap();

    (all_beacons.len(),max_dist)
}

fn parse(input: &str) -> Vec<Scanner> {
    let mut result = Vec::new();
    for (id,scanner_text) in input.split("\n\n").enumerate() {
        let mut beacons = Vec::new();

        for line in scanner_text.lines().skip(1) {
            let mut nums = Vec::new();
            for num_text in line.trim().split(',') {
                let num = num_text.parse::<i32>().unwrap();
                nums.push(num);
            }
            beacons.push(Pos(nums[0], nums[1], nums[2]));
        }

        result.push(Scanner {
            beacons,
            pos: Pos(0, 0, 0),
            id,
        });
    }

    result
}

fn overlaps(s1: &Scanner, s2: &Scanner) -> Option<Pos> {
    for s1b in &s1.beacons {
        for s2b in &s2.beacons {
            // presume s1b == s2b
            // project s2 into s1-space
            let offset = Pos(
                s1b.0 - s2b.0,
                s1b.1 - s2b.1,
                s1b.2 - s2b.2,
            );
            let s2 = s2.transl(&offset);

            // find overlaps
            let s1sees: HashSet<_> = s2
                .beacons
                .iter()
                .filter(|b| s1.can_see(b))
                .cloned()
                .collect();
            let s2sees: HashSet<_> = s1
                .beacons
                .iter()
                .filter(|b| s2.can_see(b))
                .cloned()
                .collect();

            // if s1over != s2over, continue
            if s1sees != s2sees {
                continue;
            }

            // if < 12, continue
            if s1sees.len() < 12 {
                continue;
            }

            return Some(offset);
        }
    }
    None
}

fn do_overlap(s1: &Scanner, s2: &Scanner) -> Option<Scanner> {
    for s2 in s2.orientations()
    {
        if let Some(offset) = overlaps(s1, &s2) {
            return Some(s2.transl(&offset));
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_overlap() {
        let scanners = parse(EXAMPLE);
        let overlap = do_overlap(&scanners[0], &scanners[1]);
        assert!(overlap.is_some());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 79);
    }

    static EXAMPLE: &'static str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    static EXAMPLE2: &'static str = "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8";

    #[test]
    fn test_orient() {
        let scanners = parse(EXAMPLE2);
        let orientations = scanners[0].orientations();
        for scn in &scanners[1..] {
            assert!(orientations.contains(scn));
        }
    }

    #[test]
    fn test_orient_2() {
        let s = Scanner {
            pos: Pos(0,0,0),
            beacons: vec![Pos(1,2,3)],
            id: 0,
        };
        let actual: HashSet<_> = s.orientations().into_iter()
        .map(|s| s.beacons.into_iter()).flatten().collect();
        let expected: HashSet<_> = vec![
            Pos(1,2,3),
            Pos(1,3,-2),
            Pos(1,-2,-3),
            Pos(1,-3,2),
            
            Pos(2,-1,3),
            Pos(2,-3,-1),
            Pos(2,1,-3),
            Pos(2,3,1),

            Pos(3,2,-1),
            Pos(3,-1,-2),
            Pos(3,-2,1),
            Pos(3,1,2),

            Pos(-1,-2,3),
            Pos(-1,-3,-2),
            Pos(-1,2,-3),
            Pos(-1,3,2),

            Pos(-2,1,3),
            Pos(-2,3,-1),
            Pos(-2,-1,-3),
            Pos(-2,-3,1),

            Pos(-3,-1,2),
            Pos(-3,2,1),
            Pos(-3,1,-2),
            Pos(-3,-2,-1),
        ].into_iter().collect(); 
        
        for pos in &actual {
            assert!(expected.contains(&pos));
        }
        assert_eq!(expected, actual);
    }
}
