mod assets;
mod pool;
mod positions;

pub use assets::{Asset, describe_asset};
pub use pool::Pool;
pub use positions::{Position, PositionStatus};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assets::Asset;

    fn make_pool() -> Pool {
        Pool {
            asset_a: Asset::Native {
                symbol: "SOL".to_string(),
            },
            asset_b: Asset::Erc20 {
                symbol: "USDC".to_string(),
                address: "EPjF...".to_string(),
            },
            reserve_a: 1_000_000,
            reserve_b: 50_000_000,
            positions: vec![],
        }
    }

    #[test]
    fn quote_returns_correct_amount() {
        let pool = make_pool();

        let q = pool.quote(1_000, &pool.asset_a);

        assert!(q.is_some());
        assert_eq!(q.unwrap(), 49_950);
    }

    #[test]
    fn quote_returns_none_for_unknown_asset() {
        let pool = make_pool();
        let eth = Asset::Native {
            symbol: "ETH".to_string(),
        };

        let q = pool.quote(1_000, &eth);

        assert!(q.is_none());
    }
}
