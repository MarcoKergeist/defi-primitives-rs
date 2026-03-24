#[derive(PartialEq)]
pub enum Asset {
    Native { symbol: String },
    Erc20 { symbol: String, address: String },
    LpToken(Box<Asset>, Box<Asset>),
}

pub fn describe_asset(asset: &Asset) -> String {
    match asset {
        Asset::Native { symbol } => format!("Native {symbol}"),
        Asset::Erc20 { symbol, address } => format!("ERC20 token {symbol} at {address}"),
        Asset::LpToken(a, b) => {
            let name_a = describe_asset(a);
            let name_b = describe_asset(b);
            format!("LP: {name_a}/{name_b}")
        }
    }
}
