
pub enum LengthConstraintType {
    Direct,
    Aligned,
}

pub struct LengthConstraint {
    pub length: f64,
    pub p1: usize,
    pub p2: usize,
    pub constraint_type: LengthConstraintType,
}
