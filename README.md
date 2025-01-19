# Technical indicators example implementations

This project contains examples on how to implement certain technical indicator formulas in Rust code. The indicators are: Simple Moving Average (SMA), Exponential Moving Average (EMA), Moving Average Convergence Divergence (MACD), Bollinger Bands (BOLL), and Relative Strength Index (RSI).

The project also downloads historical coin price data for BTC and USDT from the Binance API to use with these technical indicators.

I have a blog post explaining the writing of the code for this project: (How to: technical indicators with Rust and Binance)[https://tms-dev-blog.com/how-to-technical-indicators-with-rust-and-binance/]

## Running

Simply run using `cargo run --release`.


Objective
// Basic analysis to quickly note which tokens are worth looking deeper
- RSI > 70 
- RSI < 30
- MACD cross line under 0 (supported with 200 day ema for trend direction, if direction is sidelines ignore)

// CLI

// TODO:
// - Look for cross between moving averages from low to up
// - Look for RSI bigger than 70 and lower than 30
// - Take data from json file
// - Note 

// - When signals are buy, calculate entry, take profit and stop loss

// TODO:
// - Extras, store orders and balances

- TODO: take config from ~/.config

TODO: deamon that manages alerts and so on


Example config

```json
{
    "tokens": [
        {
            "key": "BTCUSDT",
            "url": "https://jup.ag/swap/USDC-SOL"
        }
    ],
    "strategy": {
        "stop_loss": "0.05",
        "take_profit": "0.07"
    },
}

```