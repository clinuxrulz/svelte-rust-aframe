#[derive(Copy, Clone)]
pub enum PolyError {
    Good = 0,
    NotClosed = 1,
    NotCoplaner = 2,
    SelfIntersecting = 3,
    ZeroLenEdge = 4,
}

pub enum StripplePattern {
    Continuous = 0,
    ShortDash = 1,
    Dash = 2,
    LongDash = 3,
    DashDot = 4,
    DashDotDot = 5,
    Dot = 6,
    Freehand = 7,
    Zigzag = 8,
}

const STRIPPLE_PATTERN_LAST: StripplePattern = StripplePattern::Zigzag;

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct HGroup {
    pub v: u32,
}

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct HRequest {
    pub v: u32,
}

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct HParam {
    pub v: u32,
}

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct HStyle {
    pub v: u32,
}

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct EntityId {
    pub v: u32,
}

pub struct EntityKey {
    pub input: HEntity,
    pub copy_number: i32,
}

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct HEntity {
    pub v: u32,
}

pub struct Param {
    pub tag: i32,
    pub h: HParam,
    pub val: f64,
    pub known: bool,
    pub free: bool,
    pub substd: HParam,
}

const MAX_POINTS_IN_ENTITY: usize = 12;

pub enum EntityBaseType {
    PointIn3D = 2000,
    PointIn2D = 2001,
    PointNTrans = 2010,
    PointNRotTrans = 2011,
    PointNCopy = 2012,
    PointNRotAA = 2013,
    PointNRotAxisTrans = 2014,

    NormalIn3D = 3000,
    NormalIn2D = 3001,
    NormalNCopy = 3010,
    NormalNRot = 3011,
    NormalRotAA = 3012,

    Distance = 4000,
    DistanceNCopy = 4001,

    FACE_NORMAL_PT = 5000,
    FACE_XPROD = 5001,
    FACE_N_ROT_TRANS = 5002,
    FACE_N_TRANS = 5003,
    FACE_N_ROT_AA = 5004,
    FACE_ROT_NORMAL_PT = 5005,
    FACE_N_ROT_AXIS_TRANS = 5006,

    WORKPLANE = 10000,
    LINE_SEGMENT = 11000,
    CUBIC = 12000,
    CUBIC_PERIODIC = 12001,
    CIRCLE = 13000,
    ARC_OF_CIRCLE = 14000,
    TTF_TEXT = 15000,
    IMAGE = 16000,
}

pub struct EntityBase {
    pub tag: i32,
    pub h: HEntity,
}
