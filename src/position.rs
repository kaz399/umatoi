//! https://toio.github.io/toio-spec/docs/hardware_position_id

use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
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
        Self { x: 0, y: 0 }
    }

    pub fn distance(&self, p: &Self) -> u16 {
        let pd = *self - *p;
        let square_f64 = ((pd.x * pd.x) + (pd.y * pd.y)) as f64;
        square_f64.sqrt().round() as u16
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MatRect {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ToioMat {
    NoMat,
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
            ToioMat::NoMat => MatRect {
                top_left: Point { x:0, y:0 },
                bottom_right: Point { x: u16::MAX, y: u16::MAX },
            },
            ToioMat::ToioCollectionMatRing => MatRect {
                top_left: Point { x: 45, y: 45 },
                bottom_right: Point { x: 455, y: 455 },
            },
            ToioMat::ToioCollectionMatColoredTiles => MatRect {
                top_left: Point { x: 545, y: 45 },
                bottom_right: Point { x: 955, y: 455 },
            },
            ToioMat::PicotonsPlayMatFront => MatRect {
                top_left: Point { x: 59, y: 2088 },
                bottom_right: Point {
                    x: 437,
                    y: 2285,
                },
            },
            ToioMat::PicotonsPlayMatBack => MatRect {
                top_left: Point { x: 59, y: 2303 },
                bottom_right: Point {
                    x: 437,
                    y: 2499,
                },
            },
            ToioMat::PicotonsControlMat => MatRect {
                top_left: Point {
                    x: 764,
                    y: 2093,
                },
                bottom_right: Point {
                    x: 953,
                    y: 2290,
                },
            },
            ToioMat::PicotonsAutoplayMat => MatRect {
                top_left: Point {
                    x: 554,
                    y: 2093,
                },
                bottom_right: Point {
                    x: 742,
                    y: 2290,
                },
            },
            ToioMat::SimpleMat => MatRect {
                top_left: Point { x: 98, y: 142 },
                bottom_right: Point { x: 402, y: 358 },
            },
            ToioMat::GesundroidMat => MatRect {
                top_left: Point { x: 1050, y: 45 },
                bottom_right: Point {
                    x: 1460,
                    y: 455,
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
            mat: ToioMat::NoMat,
        }
    }
}

impl MatPosition {
    pub fn new_with_point(point: Point) -> Self {
        Self {
            point,
            mat: ToioMat::NoMat,
        }
    }

    pub fn new_with_mat(mat: ToioMat) -> Self {
        Self {
            point: Point::origin(),
            mat,
        }
    }

    pub fn new_with_point_mat(point: Point, mat: ToioMat) -> Self {
        Self {
            point,
            mat,
        }
    }

    pub fn absorite_point(&self) -> Point {
        self.point + self.mat.rect().top_left
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position_1() {
        let p1: Point = Point { x: 10, y: 20 };
        let p2: Point = Point { x: 20, y: 40 };
        assert_eq!(p2 - p1, p1);
    }

    #[test]
    fn position_2() {
        let p1: Point = Point { x: 10, y: 10 };
        let p2: Point = Point { x: 20, y: 20 };
        let distance = p1.distance(&p2);
        println!("{}", distance);
        assert_eq!(p2 - p1, p1);
    }
}
