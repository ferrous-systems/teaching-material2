#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn from_pair(pair: (i32, i32)) -> Point {
        Point {
            x: pair.0,
            y: pair.1,
        }
    }

    fn into_pair(self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn inspect(&self) {
        println!("Current point value: {:?}", self);
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn x(&self) -> &i32 {
        &self.x
    }

    fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }

    fn y(&self) -> &i32 {
        &self.y
    }

    fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }
}

fn main() {
    let mut my_point = Point::new(1, 2);
    my_point.inspect();
    my_point.move_to(2, 3);
    my_point.inspect();

    let x = my_point.x_mut();
    *x = 5;

    my_point.inspect();
}
