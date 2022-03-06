use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use puzzle::Puzzle;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Object {
    UFO,
    AzureCow,
    YellowCow,
    PurpleCow,
    OrangeCow,
    RedBull,
    Barn,
    Crop,
    Hay,
    Fence,
    Silo,
    Wall1,
    Wall2,
    Corner,
    Empty,
}
impl Object {
    pub fn is_cow(&self) -> bool {
        matches!(
            self,
            Object::AzureCow | Object::YellowCow | Object::PurpleCow | Object::OrangeCow
        )
    }

    pub fn is_bull(&self) -> bool {
        matches!(self, Object::RedBull)
    }

    pub fn is_ufo(&self) -> bool {
        matches!(self, Object::UFO)
    }
    pub fn is_wall(&self) -> bool {
        matches!(
            self,
            Object::Barn
                | Object::Hay
                | Object::Crop
                | Object::Fence
                | Object::Wall1
                | Object::Wall2
        )
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Object::UFO => "U",
            Object::AzureCow => "A",
            Object::YellowCow => "Y",
            Object::PurpleCow => "P",
            Object::OrangeCow => "O",
            Object::RedBull => "R",
            Object::Barn => "B",
            Object::Crop => "C",
            Object::Hay => "H",
            Object::Fence => "F",
            Object::Silo => "S",
            Object::Wall1 => "|",
            Object::Wall2 => "-",
            Object::Corner => "+",
            Object::Empty => " ",
        })
    }
}

/// The `Farm` type represents the initial and fixed elements of an
/// https://www.thinkfun.com/products/invasion-of-the-cow-snatchers/[Invasion of
/// the Cow Snatchers] puzzle: the initial positions of the UFO and the cattle
/// and the fixed positions of the walls and the silo.
mod farm {
    use super::Object;
    use super::Pos;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct Farm {
        layout: [[Option<Object>; 7]; 7],
        ufo_pos: Pos,
        cow_count: i32,
    }

    impl Farm {
        pub(super) fn new() -> Self {
            Farm {
                layout: [[None; 7]; 7],
                ufo_pos: Pos::new(0, 0),
                cow_count: 0,
            }
        }

        /// Returns a reference to the gameboard at position `pos`.
        pub(super) fn get(&self, pos: Pos) -> Option<Object> {
            let (x, y) = pos.xy();
            self.layout[x][y].clone()
        }
        /// Returns a mutable reference to the gameboard at position `pos`.
        pub(super) fn get_mut(&mut self, pos: Pos) -> &mut Option<Object> {
            let (x, y) = pos.xy();
            &mut self.layout[x][y]
        }

        pub(super) fn current_ufo_pos(&self) -> Pos {
            self.ufo_pos
        }

        pub(super) fn current_ufo_pos_mut(&mut self) -> &mut Pos {
            &mut self.ufo_pos
        }

        pub(super) fn get_cow_count(&self) -> i32 {
            self.cow_count
        }

        pub(super) fn get_cow_count_mut(&mut self) -> &mut i32 {
            &mut self.cow_count
        }
    }
}
pub use self::farm::Farm;

impl Display for Farm {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for pos in Pos::values() {
            match self.get(pos) {
                None => f.write_str(" ")?,
                Some(obj) => obj.fmt(f)?,
            }
            if pos.xy().0 == 6 {
                f.write_str("\n")?
            }
        }
        Ok(())
    }
}

pub struct FarmParseError;
impl FromStr for Farm {
    type Err = FarmParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut farm = Farm::new();
        let mut c = s.chars();
        let mut ufo = false;
        let mut azure_cow = false;
        let mut yellow_cow = false;
        let mut purple_cow = false;
        let mut orange_cow = false;
        let mut red_bull = false;
        let mut silo = false;
        let mut wall_count: i32 = 0;

        // TODO ERROR CHECKING ---------------------------------------//////////////
        // e.g., too few characters, too many characters, unrecognized character,
        // unexpected template or object character,
        //missing red bull, duplicate cattle, missing or duplicate ufo, duplicate silo
        for x in 0..7 {
            for y in 0..7 {
                let pos = Pos::new(x, y);
                let put_object_with_flag = |farm: &mut Farm, flag: &mut bool, obj: Object| {
                    *farm.get_mut(pos) = Some(obj);
                    if *flag {
                        return Err(FarmParseError);
                    } else {
                        *flag = true;
                    }
                    if obj == Object::UFO {
                        *farm.current_ufo_pos_mut() = pos;
                    }
                    if obj.is_cow() {
                        *farm.get_cow_count_mut() += 1
                    }
                    Ok(())
                };
                let put_object = |farm: &mut Farm, obj: Object, wall_count: &mut i32| {
                    *farm.get_mut(pos) = Some(obj);
                    if obj == Object::UFO {
                        *farm.current_ufo_pos_mut() = pos;
                    }
                    if obj.is_wall() {
                        *wall_count = *wall_count + 1;
                    }
                    Ok(())
                };
                match c.next() {
                    None => return Err(FarmParseError),
                    Some(c) => match c {
                        'U' => put_object_with_flag(&mut farm, &mut ufo, Object::UFO)?,
                        'A' => put_object_with_flag(&mut farm, &mut azure_cow, Object::AzureCow)?,
                        'Y' => put_object_with_flag(&mut farm, &mut yellow_cow, Object::YellowCow)?,
                        'P' => put_object_with_flag(&mut farm, &mut purple_cow, Object::PurpleCow)?,
                        'O' => put_object_with_flag(&mut farm, &mut orange_cow, Object::OrangeCow)?,
                        'R' => put_object_with_flag(&mut farm, &mut red_bull, Object::RedBull)?,
                        'B' => put_object(&mut farm, Object::Barn, &mut wall_count)?,
                        'C' => put_object(&mut farm, Object::Crop, &mut wall_count)?,
                        'H' => put_object(&mut farm, Object::Hay, &mut wall_count)?,
                        'F' => put_object(&mut farm, Object::Fence, &mut wall_count)?,
                        'S' => put_object_with_flag(&mut farm, &mut silo, Object::Silo)?,
                        '|' => put_object(&mut farm, Object::Wall1, &mut wall_count)?,
                        '-' => put_object(&mut farm, Object::Wall2, &mut wall_count)?,
                        '+' => put_object(&mut farm, Object::Corner, &mut wall_count)?,
                        ' ' => put_object(&mut farm, Object::Empty, &mut wall_count)?,
                        _ => return Err(FarmParseError),
                    },
                }
            }
            match c.next() {
                Some('\n') => (),
                _ => return Err(FarmParseError),
            }
        }
        if c.next().is_some() {
            return Err(FarmParseError);
        }
        if red_bull == false || ufo == false {
            return Err(FarmParseError);
        }
        if wall_count != 24 {
            return Err(FarmParseError);
        }

        Ok(farm)
    }
}

/// The `Direction` type represents the cardinal directions, in which the UFO
/// may be moved.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    West,
    East,
}
impl Direction {
    /// An iterator over all directions.
    fn values() -> impl Iterator<Item = Self> {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .into_iter()
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Direction::North => "↑",
            Direction::South => "↓",
            Direction::West => "←",
            Direction::East => "→",
        })
    }
}

/// The (private) `pos` module ensures that the `Pos` type can only be created
/// via the `new` associated function and accessed via the `x`, `y`, and `xy`
/// methods.
mod pos {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    pub struct Pos {
        x: usize,
        y: usize,
    }
    impl Pos {
        pub fn new(x: usize, y: usize) -> Self {
            if x > 6 {
                panic!("Pos::new x (is {}) should be less than 7", x)
            }
            if y > 6 {
                panic!("Pos::new y (is {}) should be less than 7", y)
            }
            Pos { x, y }
        }
        pub fn xy(&self) -> (usize, usize) {
            (self.x, self.y)
        }
    }
}
use self::pos::Pos;

impl Pos {
    /// Return `Some(pos)` if `pos` is the position on the gameboard that is one
    /// step from `self` in the direction `dir`.  Returns `None` if there is no
    /// position on the gameboard that is one step from `self` in the direction
    /// `dir` (i.e., would move off the edge of the gameboard).
    pub fn step(&self, dir: Direction) -> Option<Self> {
        let (x, y) = self.xy();
        let (x, y) = match dir {
            Direction::North => {
                if x == 0 {
                    return None;
                }
                (x - 1, y)
            }
            Direction::South => {
                if x == 6 {
                    return None;
                }
                (x + 1, y)
            }
            Direction::West => {
                if y == 0 {
                    return None;
                }
                (x, y - 1)
            }
            Direction::East => {
                if y == 6 {
                    return None;
                }
                (x, y + 1)
            }
        };
        Some(Pos::new(x, y))
    }
    /// An iterator over all positions of the gameboard.
    pub fn values() -> impl Iterator<Item = Self> {
        (0..7).flat_map(|y| (0..7).map(move |x| Pos::new(x, y)))
    }
}
impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (x, y) = self.xy();
        write!(f, "({},{})", x, y)
    }
}

/// The `IotCS` type represents an
/// https://www.thinkfun.com/products/invasion-of-the-cow-snatchers/[Invasion of
/// the Cow Snatchers] puzzle state: a _reference_ to a `Farm` value (providing
/// information about the initial positions of the cattle and the fixed
/// positions of the walls and the silo) and the current status of the UFO
/// (position and collection of beamed-up cattle).
///  Because the `IotCS` type
/// implements `Clone`, it is important that the `IotCS` type contain a
/// _reference_ to a `Farm` value rather than an owned `Farm` value; the former
/// is simply a pointer that implments `Copy` and may therefore be efficiently
/// cloned, while the latter would require a deep copy to be cloned.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IotCS<'a> {
    farm: &'a Farm,
    ufo_pos: Pos,
    cow_collection: VecDeque<Object>, // Your code here
    red_bull_picked: bool,
}
impl<'a> IotCS<'a> {
    pub fn new(farm: &'a Farm) -> Self {
        // Your code here
        return IotCS {
            farm: farm,
            ufo_pos: farm.current_ufo_pos(),
            cow_collection: VecDeque::new(),
            red_bull_picked: false,
        };
    }

    pub fn current_ufo_pos(&self) -> Pos {
        self.ufo_pos
    }

    pub fn current_ufo_pos_mut(&mut self) -> &mut Pos {
        &mut self.ufo_pos
    }

    pub fn ufo_with_cattle_to_string(&self) -> String {
        let mut s = String::from("U:");

        for ob in self.cow_collection.iter() {
            match ob {
                Object::AzureCow => s.push('A'),
                Object::YellowCow => s.push('Y'),
                Object::PurpleCow => s.push('P'),
                Object::OrangeCow => s.push('O'),
                Object::RedBull => s.push('R'),
                _ => return s,
            }
        }

        return s;
    }

    fn move_ufo(&mut self, pos: Pos, dir: Direction) -> Option<Pos> {
        let pos1 = pos.step(dir)?;
        let obj1 = self.farm.get(pos1)?;
        // check with stack length with wall height
        match obj1 {
            Object::Barn => {
                if self.cow_collection.len() > 0 {
                    return None;
                }
            }
            Object::Crop => {
                if self.cow_collection.len() > 1 {
                    return None;
                }
            }
            Object::Fence => {
                if self.cow_collection.len() > 2 {
                    return None;
                }
            }
            Object::Hay => {
                if self.cow_collection.len() > 3 {
                    return None;
                }
            }
            _ => {}
        }
        let pos2 = pos1.step(dir)?;
        let obj2 = self.farm.get(pos2)?;
        match obj2 {
            Object::Silo => return None,
            Object::RedBull => {
                // if red bull is already picked we will have different cow count and cow collection.
                // so we check if red bull is not picked and only then compare.
                if !self.red_bull_picked
                    && self.farm.get_cow_count() as usize != self.cow_collection.len()
                {
                    return None;
                } else {
                    if !self.cow_collection.contains(&Object::RedBull) {
                        self.red_bull_picked = true;
                        self.cow_collection.push_back(Object::RedBull);
                    }
                }
            }
            Object::OrangeCow | Object::PurpleCow | Object::AzureCow | Object::YellowCow => {
                if !self.cow_collection.contains(&obj2) {
                    self.cow_collection.push_back(obj2);
                }
            }
            _ => {}
        }
        self.ufo_pos = pos2;
        Some(pos2)
    }
}

impl<'a> Puzzle for IotCS<'a> {
    type Move = Direction;

    fn is_goal(&self) -> bool {
        if (self.farm.get_cow_count() + 1) as usize != self.cow_collection.len() {
            return false;
        }
        let (x, y) = self.ufo_pos.xy();
        if x == 6 as usize || x == 0 as usize || y == 0 as usize || y == 6 as usize {
            return true;
        } else {
            return false;
        }
    }

    fn next(&self) -> Vec<(Self::Move, Self)> {
        // Your code here
        let mut next = Vec::new();

        for dir in Direction::values() {
            let mut nb = self.clone();

            if nb.move_ufo(self.ufo_pos, dir).is_some() {
                next.push((dir, nb));
            }
        }

        next
    }
}

#[cfg(test)]
mod tests;
