use num_traits::Num;

#[derive(Clone, Debug, Hash, Eq)]
pub struct Point<T: Num> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Debug, Hash, Eq)]
pub struct Rect<T: Num> {
    pub upper_left: Point<T>,
    pub width: T,
    pub height: T,
}

#[derive(Clone, Debug, Hash, Eq)]
pub struct Circle<T: Num> {
    pub center: Point<T>,
    pub radius: T,
}

#[derive(Clone, Debug, Hash, Eq)]
pub enum Figure<T: Num> {
    Circle(Circle<T>),
    Rect(Rect<T>),
}

impl<T: Num + Copy + PartialOrd> Rect<T> {
    pub fn contains(&self, p: &Point<T>) -> bool {
        self.upper_left.x <= p.x && p.x <= self.upper_left.x + self.width &&
            self.upper_left.y <= p.y && p.y <= self.upper_left.y + self.height
    }
}

impl<T: Num + Copy + Into<f64>> Rect<T> {
    pub fn area(&self) -> f64 {
        self.width.into() * self.height.into()
    }
}

impl<T: Num + Copy + PartialOrd> Circle<T> {
    pub fn contains(&self, p: &Point<T>) -> bool {
        let dx = self.center.x - p.x;
        let dy = self.center.y - p.y;
        dx * dx + dy * dy <= self.radius * self.radius
    }
}

impl<T: Num + Copy + Into<f64>> Circle<T> {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.into() * self.radius.into()
    }
}

impl<T: Num + Copy + PartialOrd> Figure<T> {
    pub fn contains(&self, p: &Point<T>) -> bool {
        match self {
            Figure::Rect(r) => r.contains(p),
            Figure::Circle(c) => c.contains(p),
        }
    }
}

impl<T: Num + Copy + Into<f64>> Figure<T> {
    pub fn area(&self) -> f64 {
        match self {
            Figure::Rect(r) => r.area(),
            Figure::Circle(c) => c.area(),
        }
    }
}

impl<T: Num> Default for Point<T> {
    fn default() -> Self {
        return Point { x: T::zero(), y: T::zero() };
    }
}

impl<T: Num> Default for Rect<T> {
    fn default() -> Self {
        return Rect { upper_left: Point::default(), width: T::one(), height: T::one() };
    }
}

impl<T: Num> Default for Circle<T> {
    fn default() -> Self {
        return Circle { center: Point::default(), radius: T::one() };
    }
}

impl<T: Num> Default for Figure<T> {
    fn default() -> Self {
        return Figure::Circle(Circle::default());
    }
}

impl<T: Num> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl<T: Num> PartialEq for Circle<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.center == other.center && self.radius == other.radius;
    }
}

impl<T: Num> PartialEq for Rect<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.upper_left == other.upper_left && self.width == other.width && self.height == other.height;
    }
}

impl<T: Num> PartialEq for Figure<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Figure::Circle(c1) => {
                if let Figure::Circle(c2) = other { c1 == c2 } else { false }
            }
            Figure::Rect(r1) => {
                if let Figure::Rect(r2) = other { r1 == r2 } else { false }
            }
        }
    }
}