pub mod data_type {
    pub struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        pub fn new(x: T, y: T) -> Point<T> {
            Point{x,y}
        }

        pub fn x(&self) -> &T {
            &self.x
        }

        pub fn y(&self) -> &T {
            &self.y
        }
    }
}

pub mod distance {
    use crate::data_type::Point;

    impl Point<f32> {
        pub fn distance_from_origin(&self) -> f32 {
            (self.x().powi(2) + self.y().powi(2)).sqrt()
        }
    }
}
