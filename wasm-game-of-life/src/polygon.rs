enum EarType {
    Unknown,
    NotEar,
    Ear,
}

enum BspClass {
    Pos,
    Neg,
    Coplaner,
}

enum EdgeKind {
    NakedOrSelfInter,
    SelfInter,
    Turning,
    Emphasized,
    Sharp,
}

pub struct SEdge {
    pub tag: i32,
    pub auxA: i32,
    pub auxB: i32,
}
