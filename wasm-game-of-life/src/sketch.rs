use crate::dsc::{Quaternion, RgbaColor};
use std::collections::HashMap;

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

#[derive(Copy, Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct EntityKey {
    pub input: HEntity,
    pub copy_number: i32,
}

pub type EntityMap = HashMap<EntityKey, EntityId>;

pub enum GroupCopyAs {
    Numeric,
    NTrans,
    NRotAA,
    NRotTrans,
    NRotAxisTrans,
}

pub enum GroupType {
    Drawing3D = 5000,
    DrawingWorkplane = 5001,
    Extrude = 5100,
    Lathe = 5101,
    Revolve = 5102,
    Helix = 5103,
    Rotate = 5200,
    Translate = 5201,
    Linked = 5300,
}

pub struct GroupSolved {
    pub how: SolveResult,
    pub dof: i32,
    pub find_to_fix_timeout: i32,
    pub timeout: bool,
    pub remove: Vec<HConstraint>,
}

pub enum GroupSubtype {
    WorkplaneByPointOrtho = 6000,
    WorkplaneByLineSegments = 6001,
    OneSided = 7000,
    TwoSided = 7001,
}

pub struct GroupPredef {
    pub q: Quaternion,
    pub origin: HEntity,
    pub entity_b: HEntity,
    pub entity_c: HEntity,
    pub swap_uv: bool,
    pub negate_u: bool,
    pub negate_v: bool,
}

pub struct GroupPolyError {
    pub how: PolyError,
    pub not_closed_at: SEdge,
    error_point_at: Vector,
}

pub enum GroupCombineAs {
    Union = 0,
    Difference = 1,
    Assemble = 2,
    Intersection = 3,
}

pub enum GroupRemap {
    Last = 1000,
    Top = 1001,
    Bottom = 1002,
    PtToLine = 1003,
    LineToFace = 1004,
    LatheStart = 1006,
    LatheEnd = 1007,
    PtToArc = 1008,
    PtToNormal = 1009,
    LatheArcCenter = 1010,
}

pub struct Group {
    pub tag: i32,
    pub h: HGroup,
    pub type_: GroupType,
    pub order: i32,

    pub op_a: HGroup,
    pub op_b: HGroup,
    pub visible: bool,
    pub suppress: bool,
    pub relaxConstraints: bool,
    pub allow_redundant: bool,
    pub all_dims_reference: bool,
    pub scale: f64,

    pub clean: bool,
    pub dof_check_ok: bool,
    pub active_workplane: HEntity,
    pub val_a: f64,
    pub val_b: f64,
    pub val_c: f64,
    pub color: RgbaColor,

    pub solved: GroupSolved,
    pub subtype: GroupSubtype,

    pub skip_first: bool,

    pub predef: GroupPredef,
    pub poly_loops: SPolygon,
    pub bezier_loops: SBezierLoopSetSet,
    pub bezier_opens: SBezierLoopSet,
    pub poly_error: GroupPolyError,

    pub boolean_failed: bool,

    pub this_shell: SShell,
    pub running_shell: SShell,

    pub this_mesh: SMesh,
    pub running_mesh: SMesh,

    pub display_dirty: bool,
    pub display_mesh: SMesh,
    pub display_outlines: SOutlineList,
    pub mesh_combine: GroupCombineAs,

    pub force_to_mesh: bool,

    pub remap: EntityMap,

    pub link_file: PlatformPath,

    pub imp_mesh: SMesh,
    pub imp_shell: SShell,
    pub imp_entity: EntityList,

    pub name: String,
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
