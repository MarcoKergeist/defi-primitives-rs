# defi-primitives

A lightweight Rust crate modeling core DeFi primitives: assets 
and liquidity pools based on the constant product formula (x·y=k).

> [!WARNING]
> Built as a learning exercise toward DeFi development.
> Not production-ready.

## What's included

- `Asset` — native tokens, ERC20/SPL tokens, and recursive LP tokens
- `Pool` — liquidity pool with two assets and position tracking  
- `Pool::quote()` — AMM output calculation using the x·y=k invariant
- `Pool::constant_product()` — returns k for a given pool state

## The math

Given a pool with reserves `x` and `y`, the invariant `k` is:

    k = x · y

For a trade of `amount_in`:

    amount_out = y · amount_in / (x + amount_in)

## Usage

```rust
let pool = Pool { ... };
let out = pool.quote(1_000, &pool.asset_a);
```

## Roadmap

- [ ] Position management (deposit, withdraw)
- [ ] Fee calculation
- [ ] Multi-hop routing

## Author

Marco Kergeist - [marcokergeist@proton.me](mailto:marcokergeist@proton.me)
