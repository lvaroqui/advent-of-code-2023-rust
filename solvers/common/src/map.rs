#[derive(Debug, Clone)]
pub struct Map<T>(Vec<Vec<T>>);

impl<T> Map<T> {
    pub fn new(map: Vec<Vec<T>>) -> Self {
        Map(map)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vec2, &T)> {
        self.0.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, val)| {
                (
                    Vec2 {
                        x: x as i64,
                        y: y as i64,
                    },
                    val,
                )
            })
        })
    }

    pub fn iter_from_point(
        &self,
        start: Vec2,
        direction: Vec2,
    ) -> impl Iterator<Item = (Vec2, &T)> {
        let mut current = start;
        std::iter::from_fn(move || {
            let value = self.get(current).map(|v| (current, v));

            current = current + direction;

            value
        })
    }

    pub fn eight_adjacent_iter(&self, pos: Vec2) -> impl Iterator<Item = &T> {
        self.eight_adjacent_pos_iter(pos)
            .map(|pos| &self.0[pos.y as usize][pos.x as usize])
    }

    pub fn eight_adjacent_pos_iter(&self, pos: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        let x = pos.x;
        let y = pos.y;
        [
            // Line above
            vec2(x - 1, y - 1),
            vec2(x, y - 1),
            vec2(x + 1, y - 1),
            // Left
            vec2(x - 1, y),
            //Right
            vec2(x + 1, y),
            // Line bellow
            vec2(x - 1, y + 1),
            vec2(x, y + 1),
            vec2(x + 1, y + 1),
        ]
        .into_iter()
        .filter(|&pos| {
            pos.x >= 0
                && pos.x < self.0[0].len() as i64
                && pos.y >= 0
                && pos.y < self.0.len() as i64
        })
    }

    pub fn four_adjacent_iter(&self, pos: Vec2) -> impl Iterator<Item = &T> {
        self.four_adjacent_pos_iter(pos)
            .map(|pos| &self.0[pos.y as usize][pos.x as usize])
    }

    pub fn four_adjacent_pos_iter(&self, pos: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        self.four_adjacent_pos_iter_unchecked(pos).filter(|&pos| {
            pos.x >= 0
                && pos.x < self.0[0].len() as i64
                && pos.y >= 0
                && pos.y < self.0.len() as i64
        })
    }

    pub fn four_adjacent_pos_iter_unchecked(&self, pos: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        let x = pos.x;
        let y = pos.y;
        [
            // Above
            vec2(x, y - 1),
            // Left
            vec2(x - 1, y),
            // Right
            vec2(x + 1, y),
            // Bellow
            vec2(x, y + 1),
        ]
        .into_iter()
    }

    pub fn get(&self, pos: Vec2) -> Option<&T> {
        self.0.get(pos.y as usize)?.get(pos.x as usize)
    }

    pub fn get_mut(&mut self, pos: Vec2) -> Option<&mut T> {
        self.0.get_mut(pos.y as usize)?.get_mut(pos.x as usize)
    }

    pub fn inner(&self) -> &Vec<Vec<T>> {
        &self.0
    }
}

impl<T> std::ops::Index<Vec2> for Map<T> {
    type Output = T;

    fn index(&self, pos: Vec2) -> &Self::Output {
        &self.0[pos.y as usize][pos.x as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

pub fn vec2(x: impl Into<i64>, y: impl Into<i64>) -> Vec2 {
    Vec2 {
        x: x.into(),
        y: y.into(),
    }
}

impl Vec2 {
    pub const EAST: Self = Vec2 { x: 1, y: 0 };
    pub const SOUTH_EAST: Self = Vec2 { x: 1, y: 1 };
    pub const SOUTH: Self = Vec2 { x: 0, y: 1 };
    pub const SOUTH_WEST: Self = Vec2 { x: -1, y: 1 };
    pub const WEST: Self = Vec2 { x: -1, y: 0 };
    pub const NORTH_WEST: Self = Vec2 { x: -1, y: -1 };
    pub const NORTH: Self = Vec2 { x: 0, y: -1 };
    pub const NORTH_EAST: Self = Vec2 { x: 1, y: -1 };

    pub fn directions() -> [Vec2; 8] {
        [
            Vec2::EAST,
            Vec2::SOUTH_EAST,
            Vec2::SOUTH,
            Vec2::SOUTH_WEST,
            Vec2::WEST,
            Vec2::NORTH_WEST,
            Vec2::NORTH,
            Vec2::NORTH_EAST,
        ]
    }

    pub fn rotate(self, angle: f32) -> Self {
        let x = self.x as f32;
        let y = self.y as f32;
        let x_rotated = (x * angle.cos()) - (y * angle.sin());
        let y_rotated = (y * angle.cos()) + (x * angle.sin());
        vec2(x_rotated as i64, y_rotated as i64)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<i64> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: i64) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub<i64> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: i64) -> Self::Output {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl std::ops::Sub<Vec2> for i64 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self - rhs.x,
            y: self - rhs.y,
        }
    }
}

impl std::ops::Mul<i64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<Vec2> for i64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs.mul(self)
    }
}
