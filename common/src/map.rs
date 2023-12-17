pub struct Map<T>(Vec<Vec<T>>);

impl<T> Map<T> {
    pub fn new(map: Vec<Vec<T>>) -> Self {
        Map(map)
    }
}

impl<T> Map<T> {
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

    pub fn adjacent_iter(&self, pos: Vec2) -> impl Iterator<Item = &T> {
        self.adjacent_pos_iter(pos)
            .map(|pos| &self.0[pos.y as usize][pos.x as usize])
    }

    pub fn adjacent_pos_iter(&self, pos: Vec2) -> impl Iterator<Item = Vec2> + '_ {
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

    pub fn get(&self, pos: Vec2) -> Option<&T> {
        self.0.get(pos.y as usize)?.get(pos.x as usize)
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
    pub const LEFT: Self = Vec2 { x: -1, y: 0 };
    pub const RIGHT: Self = Vec2 { x: 1, y: 0 };
    pub const UP: Self = Vec2 { x: 0, y: -1 };
    pub const DOWN: Self = Vec2 { x: 0, y: 1 };
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
