use num::Signed;

pub trait Coordinates<T: Signed> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GCSPoint {
    pub x: f64,
    pub y: f64,
}

impl Coordinates<f64> for GCSPoint {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl From<(f64, f64)> for GCSPoint {
    fn from(value: (f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl ToString for GCSPoint {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct XYPoint {
    pub x: i64,
    pub y: i64,
}

impl Coordinates<i64> for XYPoint {
    fn x(&self) -> i64 {
        self.x
    }

    fn y(&self) -> i64 {
        self.y
    }
}

impl From<(i64, i64)> for XYPoint {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl ToString for XYPoint {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Point {
    GCS(GCSPoint),
    XY(XYPoint),
}

impl Coordinates<f64> for Point {
    fn x(&self) -> f64 {
        match self {
            Self::GCS(point) => point.x,
            Self::XY(_) => panic!("XY points use i64 instead of f64."),
        }
    }

    fn y(&self) -> f64 {
        match self {
            Self::GCS(point) => point.y,
            Self::XY(_) => panic!("XY points use i64 instead of f64."),
        }
    }
}

impl Coordinates<i64> for Point {
    fn x(&self) -> i64 {
        match self {
            Self::GCS(_) => panic!("GCS points use f64 instead of i64."),
            Self::XY(point) => point.x,
        }
    }

    fn y(&self) -> i64 {
        match self {
            Self::GCS(_) => panic!("GCS points use f64 instead of i64."),
            Self::XY(point) => point.y,
        }
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        match self {
            Point::GCS(p) => format!("GCS{}", p.to_string()),
            Point::XY(p) => format!("XY{}", p.to_string()),
        }
    }
}
