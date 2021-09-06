#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug)]
struct Rect {
    upper_left: Point,
    width: f64,
    height: f64,
}

#[derive(Clone, Debug)]
struct Circle {
    center: Point,
    radius: f64,
}

#[derive(Clone, Debug)]
enum Figure {
    Circle(Circle),
    Rect(Rect),
}

impl Rect {
    fn contains(&self, p: &Point) -> bool {
        self.upper_left.x <= p.x && p.x <= self.upper_left.x + self.width &&
            self.upper_left.y <= p.y && p.y <= self.upper_left.y + self.height
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Circle {
    fn contains(&self, p: &Point) -> bool {
        let dx = self.center.x - p.x;
        let dy = self.center.y - p.y;
        dx * dx + dy * dy <= self.radius * self.radius
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Figure {
    fn contains(&self, p: &Point) -> bool {
        match self {
            Figure::Rect(r) => r.contains(p),
            Figure::Circle(c) => c.contains(p),
        }
    }
    fn area(&self) -> f64 {
        match self {
            Figure::Rect(r) => r.area(),
            Figure::Circle(c) => c.area(),
        }
    }
}


fn main() {
    let rectangle = Rect {
        upper_left: Point { x: 1.0, y: 2.0 },
        width: 5.0,
        height: 4.0,
    };
    let circle = Circle {
        center: Point { x: 5.0, y: 6.0 },
        radius: 2.0,
    };

    println!("This is a rectangle with area = {}: {:?}", rectangle.area(), rectangle);
    println!("This is a circle with area = {}: {:?}", circle.area(), circle);

    let figure1 = Figure::Rect(rectangle.clone());
    println!("This is a figure with area = {}: {:?}", figure1.area(), figure1);
    let figure2 = Figure::Circle(circle.clone());
    println!("This is a figure with area = {}: {:?}", figure2.area(), figure2);

    let points = [
        Point { x: 1.0, y: 2.0 },
        Point { x: 5.0, y: 5.0 },
    ];

    for p in points {
        let contains_str = if rectangle.contains(&p) { "contains" } else { "does not contain" };
        println!("{:?} {} the point {:?}", rectangle, contains_str, p);
        let contains_str = if circle.contains(&p) { "contains" } else { "does not contain" };
        println!("{:?} {} the point {:?}", circle, contains_str, p);
        let contains_str = if figure1.contains(&p) { "contains" } else { "does not contain" };
        println!("{:?} {} the point {:?}", figure1, contains_str, p);
        let contains_str = if figure2.contains(&p) { "contains" } else { "does not contain" };
        println!("{:?} {} the point {:?}", figure2, contains_str, p);
    }
}
