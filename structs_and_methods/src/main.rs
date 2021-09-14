use structs_and_methods::{Point, Rect, Circle, Figure};

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

    println!("These are the default shapes:");
    print!("{:?}\n{:?}\n{:?}\n{:?}\n",
           Point::<f64>::default(),
           Rect::<f64>::default(),
           Circle::<f64>::default(),
           Figure::<f64>::default())
}
