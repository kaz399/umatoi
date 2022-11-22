//! Official Specification: <https://toio.github.io/toio-spec/docs/hardware_position_id>

use serde::Serialize;
use std::ops::{Add, Sub};
use std::convert::{From, TryFrom};

/// Point

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl From<&[isize]> for Point {
    fn from(data: &[isize]) -> Self {
        Self {
            x: data[0],
            y: data[1],
        }
    }
}

impl TryFrom<&[usize]> for Point {
    type Error = ();

    fn try_from(data: &[usize]) -> Result<Self, Self::Error> {
        if (data[0] <= isize::MAX as usize) && (data[1] <= isize::MAX as usize) {
            Ok(
                Self {
                    x: data[0] as isize,
                    y: data[1] as isize,
                }
            )
        } else {
            Err(())
        }
    }
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

    pub fn distance(self, p: Self) -> isize {
        let pd = self - p;
        let square_f64 = ((pd.x * pd.x) + (pd.y * pd.y)) as f64;
        square_f64.sqrt().round() as isize
    }

    pub fn inside(self, rect: MatRect) -> bool {
        rect.top_left <= self && self <= rect.bottom_right
    }
}

/// Location information of a cube

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct CubeLocation {
    pub point: Point,
    pub angle: u16,
}

impl Default for CubeLocation {
    fn default() -> Self {
        Self {
            point: Point::origin(),
            angle: 0,
        }
    }
}

impl Add for CubeLocation {
    type Output = Self;

    fn add(self, p: Self) -> Self {
        Self {
            point: self.point + p.point,
            angle: {
                let new_angle = self.angle + p.angle;
                if new_angle > 360 {
                    new_angle % 360
                } else {
                    new_angle
                }
            },
        }
    }
}

impl Sub for CubeLocation {
    type Output = Self;

    fn sub(self, p: Self) -> Self {
        Self {
            point: self.point - p.point,
            angle: {
                if (self.angle % 360) > (p.angle % 360) {
                    (self.angle % 360) - (p.angle % 360)
                } else {
                    360 + (self.angle % 360) - (p.angle % 360)
                }
            },
        }
    }
}

/// Mat rectangle

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MatRect {
    top_left: Point,
    bottom_right: Point,
}

impl MatRect {
    pub fn includes(self, point: Point) -> bool {
        point.inside(self)
    }
}

/// Toio mat

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    UserDefinedMat { rect: MatRect} ,
}

impl ToioMat {
    pub fn rect(&self) -> MatRect {
        match self {
            ToioMat::NoMat => MatRect {
                top_left: Point { x: 0, y: 0 },
                bottom_right: Point {
                    x: isize::MAX,
                    y: isize::MAX,
                },
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
                bottom_right: Point { x: 437, y: 2285 },
            },
            ToioMat::PicotonsPlayMatBack => MatRect {
                top_left: Point { x: 59, y: 2303 },
                bottom_right: Point { x: 437, y: 2499 },
            },
            ToioMat::PicotonsControlMat => MatRect {
                top_left: Point { x: 764, y: 2093 },
                bottom_right: Point { x: 953, y: 2290 },
            },
            ToioMat::PicotonsAutoplayMat => MatRect {
                top_left: Point { x: 554, y: 2093 },
                bottom_right: Point { x: 742, y: 2290 },
            },
            ToioMat::SimpleMat => MatRect {
                top_left: Point { x: 98, y: 142 },
                bottom_right: Point { x: 402, y: 358 },
            },
            ToioMat::GesundroidMat => MatRect {
                top_left: Point { x: 1050, y: 45 },
                bottom_right: Point { x: 1460, y: 455 },
            },
            ToioMat::UserDefinedMat { rect } =>  *rect,
        }
    }

}

/// Cube location on a toio mat

pub struct RelativeCubeLocation {
    location: CubeLocation,
    mat: ToioMat,
}

impl Default for RelativeCubeLocation {
    fn default() -> Self {
        Self {
            location: CubeLocation::default(),
            mat: ToioMat::NoMat,
        }
    }
}

impl Add for RelativeCubeLocation {
    type Output = Self;

    fn add(self, p: Self) -> Self {
        if self.mat == p.mat {
            Self {
                location: self.location + p.location,
                mat: self.mat,
            }
        } else {
            Self {
                location: self.absolute_location() + p.absolute_location(),
                mat: ToioMat::NoMat,
            }
        }
    }
}

impl Sub for RelativeCubeLocation {
    type Output = Self;

    fn sub(self, p: Self) -> Self {
        if self.mat == p.mat {
            Self {
                location: self.location - p.location,
                mat: self.mat,
            }
        } else {
            Self {
                location: self.absolute_location() - p.absolute_location(),
                mat: ToioMat::NoMat,
            }
        }
    }
}

impl RelativeCubeLocation {
    pub fn absolute_point(&self) -> Point {
        self.location.point + self.mat.rect().top_left
    }

    pub fn absolute_location(&self) -> CubeLocation {
        CubeLocation {
            point: self.location.point + self.mat.rect().top_left,
            angle: self.location.angle,
        }
    }

    pub fn from_absolute_location(&mut self, abs_location: CubeLocation) -> CubeLocation {
        self.location = CubeLocation {
            point: abs_location.point - self.mat.rect().top_left,
            angle: abs_location.angle,
        };
        self.location
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position_point1() {
        let p1: Point = Point { x: 10, y: 20 };
        let p2: Point = Point { x: 20, y: 40 };
        assert_eq!(p2 - p1, p1);
    }

    #[test]
    fn position_point2() {
        let p1: Point = Point { x: 10, y: 10 };
        let p2: Point = Point { x: 20, y: 20 };
        let distance = p1.distance(p2);
        println!("{}", distance);
        assert_eq!(p2 - p1, p1);
    }

    #[test]
    fn position_inside1() {
        let p1: Point = Point { x: 10, y: 10 };
        let mat: MatRect = MatRect {
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point { x: 15, y: 15 },
        };
        assert!(p1.inside(mat));
    }

    #[test]
    fn position_inside2() {
        let p1: Point = Point { x: 10, y: 10 };
        let mat: MatRect = MatRect {
            top_left: Point { x: 10, y: 10 },
            bottom_right: Point { x: 15, y: 15 },
        };
        assert!(p1.inside(mat));
    }

    #[test]
    fn position_inside3() {
        let p1: Point = Point { x: 10, y: 10 };
        let mat: MatRect = MatRect {
            top_left: Point { x: 11, y: 10 },
            bottom_right: Point { x: 15, y: 15 },
        };
        assert!(!p1.inside(mat));
    }

    #[test]
    fn position_inside4() {
        let p1: Point = Point { x: 15, y: 15 };
        let mat: MatRect = MatRect {
            top_left: Point { x: 10, y: 10 },
            bottom_right: Point { x: 15, y: 15 },
        };
        assert!(p1.inside(mat));
    }

    #[test]
    fn position_inside5() {
        let p1: Point = Point { x: 16, y: 15 };
        let mat: MatRect = MatRect {
            top_left: Point { x: 10, y: 10 },
            bottom_right: Point { x: 15, y: 15 },
        };
        assert!(!p1.inside(mat));
    }

    #[test]
    fn position_mat1() {
        let toio_mat = ToioMat::UserDefinedMat {
            rect: MatRect {
                top_left: Point { x: 10, y: 10 },
                bottom_right: Point { x: 30, y: 30 },
            }
        };
        let rect1 = toio_mat.rect();
        let rect2: MatRect  = MatRect {
            top_left: Point { x: 10, y: 10 },
            bottom_right: Point { x: 30, y: 30 },
        };
        assert_eq!(rect1, rect2);
        assert_ne!(rect1, ToioMat::GesundroidMat.rect());
    }

    #[test]
    fn position_mat2() {
        let p1: Point = Point { x: 1050, y: 45 };
        assert!(ToioMat::GesundroidMat.rect().includes(p1));
        let p1: Point = Point { x: 1460, y: 455 };
        assert!(ToioMat::GesundroidMat.rect().includes(p1));
    }

    #[test]
    fn position_mat3() {
        let relative_location: RelativeCubeLocation = RelativeCubeLocation {
            location: CubeLocation {
                point: Point { x: 0, y:0 },
                angle: 0 },
                mat: ToioMat::GesundroidMat,
        };
        let absolute_point = ToioMat::GesundroidMat.rect().top_left;
        assert_eq!(absolute_point, relative_location.absolute_point());
    }

}
