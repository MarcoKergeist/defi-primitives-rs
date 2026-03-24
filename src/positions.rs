use crate::assets::Asset;

pub struct Position {
    pub deposited: Asset,
    pub amount: u64,
    pub status: PositionStatus,
}

pub enum PositionStatus {
    Active,
    Liquidated,
    Withdrawn,
}
