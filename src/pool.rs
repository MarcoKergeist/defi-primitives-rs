use crate::assets::Asset;
use crate::positions::Position;

pub struct Pool {
    pub asset_a: Asset,
    pub asset_b: Asset,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub positions: Vec<Position>,
}

impl Pool {
    pub fn constant_product(&self) -> u128 {
        (self.reserve_a as u128) * (self.reserve_b as u128)
    }

    pub fn quote(&self, amount_in: u64, asset_in: &Asset) -> Option<u64> {
        let (reserve_in, reserve_out) = if *asset_in == self.asset_a {
            (self.reserve_a, self.reserve_b)
        } else if *asset_in == self.asset_b {
            (self.reserve_b, self.reserve_a)
        } else {
            return None;
        };

        let q = (reserve_out as u128) * (amount_in as u128);
        let d = (reserve_in as u128) + (amount_in as u128);

        if d == 0 { None } else { Some((q / d) as u64) }
    }
}
