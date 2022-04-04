//! https://toio.github.io/toio-spec/docs/hardware_position_id

use std::ops::{Add, Sub};

type FLOAT = f64;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: FLOAT,
    pub y: FLOAT,
}

impl Add for Point {
    type Output = Self;
    fn add(self, p: Self) -> Self {
        Self {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, p: Self) -> Self {
        Self {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }
}

impl Point {
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Point {
    pub fn distance(&self, p: &Self) -> FLOAT {
        let pd = *self - *p;
        ((pd.x * pd.x) + (pd.y * pd.y)).sqrt()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MatRect {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ToioMat {
    ToioCollectionMatRing,
    ToioCollectionMatColoredTiles,
    PicotonsPlayMatFront,
    PicotonsPlayMatBack,
    PicotonsControlMat,
    PicotonsAutoplayMat,
    SimpleMat,
    GesundroidMat,
}

impl ToioMat {
    pub fn rect(&self) -> MatRect {
        match self {
            ToioMat::ToioCollectionMatRing => MatRect {
                top_left: Point { x: 45.0, y: 45.0 },
                bottom_right: Point { x: 455.0, y: 455.0 },
            },
            ToioMat::ToioCollectionMatColoredTiles => MatRect {
                top_left: Point { x: 545.0, y: 45.0 },
                bottom_right: Point { x: 955.0, y: 455.0 },
            },
            ToioMat::PicotonsPlayMatFront => MatRect {
                top_left: Point { x: 59.0, y: 2088.0 },
                bottom_right: Point {
                    x: 437.0,
                    y: 2285.0,
                },
            },
            ToioMat::PicotonsPlayMatBack => MatRect {
                top_left: Point { x: 59.0, y: 2303.0 },
                bottom_right: Point {
                    x: 437.0,
                    y: 2499.0,
                },
            },
            ToioMat::PicotonsControlMat => MatRect {
                top_left: Point {
                    x: 764.0,
                    y: 2093.0,
                },
                bottom_right: Point {
                    x: 953.0,
                    y: 2290.0,
                },
            },
            ToioMat::PicotonsAutoplayMat => MatRect {
                top_left: Point {
                    x: 554.0,
                    y: 2093.0,
                },
                bottom_right: Point {
                    x: 742.0,
                    y: 2290.0,
                },
            },
            ToioMat::SimpleMat => MatRect {
                top_left: Point { x: 98.0, y: 142.0 },
                bottom_right: Point { x: 402.0, y: 358.0 },
            },
            ToioMat::GesundroidMat => MatRect {
                top_left: Point { x: 1050.0, y: 45.0 },
                bottom_right: Point {
                    x: 1460.0,
                    y: 455.0,
                },
            },
        }
    }
}

pub struct MatPosition {
    point: Point,
    mat: ToioMat,
}

impl Default for MatPosition {
    fn default() -> Self {
        Self {
            point: Point::origin(),
            mat: ToioMat::ToioCollectionMatRing,
        }
    }
}

impl MatPosition {
    pub fn absorite_point(&self) -> Point {
        self.point + self.mat.rect().top_left
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position_1() {
        let p1: Point = Point { x: 10.0, y: 20.0 };
        let p2: Point = Point { x: 20.0, y: 40.0 };
        assert_eq!(p2 - p1, p1);
    }

    #[test]
    fn position_2() {
        let p1: Point = Point { x: 10.0, y: 10.0 };
        let p2: Point = Point { x: 20.0, y: 20.0 };
        let distance = p1.distance(&p2);
        println!("{}", distance);
        assert_eq!(p2 - p1, p1);
    }
}
