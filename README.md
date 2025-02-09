
# Lazy trader

Lazy trader is a cli that makes it easy to quickly grasp a sense of the market. Taking some popular indicators, rsi, bb, ema, sma, etc. We quickly show to the user potential opportunities. The idea is to quickly get a sense of weather it's worth it to further analyze the situation and make a trade or not.

## Commands

```bash
lt --help

# Usage: lt <COMMAND>

# Commands:
#   sentiment   Analyze market sentiment
#   config      Configuration
#   entry       Calculate entry
#   indicators  Explain indicators
#   help        Print this message or the help of the given subcommand(s)

# Options:
#   -h, --help     Print help
#   -V, --version  Print version
```

### Sentiment

```bash
lt sentiment --help

# Analyze market sentiment

# Usage: lt sentiment [OPTIONS] --interval <INTERVAL> [MARKET]

# Arguments:
#   [MARKET]  

# Options:
#       --limit <LIMIT>        [default: 200]
#   -i, --interval <INTERVAL>  [possible values: d1, h1]
#       --hide-neutral         
#   -h, --help                 Print help
```

Example outputs

```bash
lt sentiment --interval h1

# +-----------------+---------+---------+---------+---------+
# | 08-02-2025 13hs | Neutral | Bearish | Bullish | Neutral |
# | 08-02-2025 14hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 15hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 16hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 17hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 18hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 19hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 20hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 21hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 22hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 23hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 00hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 01hs | Neutral | Bearish | Neutral | Bullish |
# | 09-02-2025 02hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 03hs | Neutral | Bearish | Neutral | Bullish |
# | 09-02-2025 04hs | Neutral | Bearish | Neutral | Bullish |
# | 09-02-2025 05hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 06hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 07hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 08hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 09hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 10hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 11hs | Neutral | Bearish | Bearish | Neutral |
# | 09-02-2025 12hs | Neutral | Bearish | Neutral | Neutral |
# | Date            | RSI     | EMA     | MACD    | BB      |
# +-----------------+---------+---------+---------+---------+
# BTCUSDT - 1H
# ------------------------------------------------------------------------------------------
# +-----------------+---------+---------+---------+---------+
# | 08-02-2025 13hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 14hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 15hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 16hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 17hs | Neutral | Bearish | Neutral | Bullish |
# | 08-02-2025 18hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 19hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 20hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 21hs | Neutral | Bearish | Neutral | Bullish |
# | 08-02-2025 22hs | Neutral | Bullish | Neutral | Neutral |
# | 08-02-2025 23hs | Neutral | Bullish | Neutral | Neutral |
# | 09-02-2025 00hs | Neutral | Bullish | Neutral | Neutral |
# | 09-02-2025 01hs | Neutral | Bullish | Neutral | Bullish |
# | 09-02-2025 02hs | Neutral | Bullish | Neutral | Neutral |
# | 09-02-2025 03hs | Bearish | Bullish | Neutral | Bullish |
# | 09-02-2025 04hs | Bearish | Bullish | Neutral | Bullish |
# | 09-02-2025 05hs | Bearish | Bullish | Neutral | Neutral |
# | 09-02-2025 06hs | Neutral | Bullish | Neutral | Neutral |
# | 09-02-2025 07hs | Neutral | Bullish | Neutral | Neutral |
# | 09-02-2025 08hs | Neutral | Bullish | Neutral | Neutral |
# | 09-02-2025 09hs | Neutral | Bullish | Neutral | Neutral |
# | 09-02-2025 10hs | Neutral | Bullish | Bearish | Neutral |
# | 09-02-2025 11hs | Neutral | Bullish | Neutral | Neutral |
# | 09-02-2025 12hs | Neutral | Bullish | Neutral | Neutral |
# | Date            | RSI     | EMA     | MACD    | BB      |
# +-----------------+---------+---------+---------+---------+
# SOLUSDT - 1H
# ------------------------------------------------------------------------------------------
# +-----------------+---------+---------+---------+---------+
# | 08-02-2025 13hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 14hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 15hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 16hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 17hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 18hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 19hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 20hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 21hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 22hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 23hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 00hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 01hs | Neutral | Bearish | Neutral | Bullish |
# | 09-02-2025 02hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 03hs | Neutral | Bearish | Neutral | Bullish |
# | 09-02-2025 04hs | Neutral | Bearish | Neutral | Bullish |
# | 09-02-2025 05hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 06hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 07hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 08hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 09hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 10hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 11hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 12hs | Neutral | Bearish | Neutral | Neutral |
# | Date            | RSI     | EMA     | MACD    | BB      |
# +-----------------+---------+---------+---------+---------+
# ETHUSDT - 1H
# ------------------------------------------------------------------------------------------
```

```bash
lt sentiment ethusdt --interval h1

# +-----------------+---------+---------+---------+---------+
# | 01-02-2025 07hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 08hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 09hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 10hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 11hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 12hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 13hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 14hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 15hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 16hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 17hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 18hs | Neutral | Neutral | Bullish | Neutral |
# | 01-02-2025 19hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 20hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 21hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 22hs | Neutral | Neutral | Neutral | Neutral |
# | 01-02-2025 23hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 00hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 01hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 02hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 03hs | Neutral | Neutral | Neutral | Bearish |
# | 02-02-2025 04hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 05hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 06hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 07hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 08hs | Neutral | Neutral | Bearish | Neutral |
# | 02-02-2025 09hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 10hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 11hs | Bullish | Neutral | Neutral | Neutral |
# | 02-02-2025 12hs | Bullish | Neutral | Neutral | Neutral |
# | 02-02-2025 13hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 14hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 15hs | Neutral | Neutral | Neutral | Neutral |
# | 02-02-2025 16hs | Bullish | Neutral | Neutral | Bearish |
# | 02-02-2025 17hs | Bullish | Neutral | Neutral | Bearish |
# | 02-02-2025 18hs | Bullish | Neutral | Neutral | Bearish |
# | 02-02-2025 19hs | Bullish | Neutral | Neutral | Neutral |
# | 02-02-2025 20hs | Bullish | Neutral | Neutral | Neutral |
# | 02-02-2025 21hs | Bullish | Neutral | Neutral | Bearish |
# | 02-02-2025 22hs | Bullish | Neutral | Neutral | Bearish |
# | 02-02-2025 23hs | Bullish | Neutral | Neutral | Neutral |
# | 03-02-2025 00hs | Bullish | Neutral | Neutral | Bearish |
# | 03-02-2025 01hs | Bullish | Neutral | Neutral | Bearish |
# | 03-02-2025 02hs | Bullish | Neutral | Neutral | Bearish |
# | 03-02-2025 03hs | Bullish | Neutral | Neutral | Bearish |
# | 03-02-2025 04hs | Bullish | Neutral | Neutral | Bearish |
# | 03-02-2025 05hs | Bullish | Neutral | Neutral | Neutral |
# | 03-02-2025 06hs | Bullish | Neutral | Neutral | Neutral |
# | 03-02-2025 07hs | Neutral | Neutral | Neutral | Neutral |
# | 03-02-2025 08hs | Neutral | Neutral | Neutral | Neutral |
# | 03-02-2025 09hs | Neutral | Bearish | Neutral | Neutral |
# | 03-02-2025 10hs | Neutral | Bearish | Neutral | Neutral |
# | 03-02-2025 11hs | Neutral | Bearish | Bullish | Neutral |
# | 03-02-2025 12hs | Neutral | Bearish | Neutral | Neutral |
# | 03-02-2025 13hs | Neutral | Bearish | Neutral | Neutral |
# | 03-02-2025 14hs | Neutral | Bearish | Neutral | Neutral |
# | 03-02-2025 15hs | Neutral | Bearish | Neutral | Neutral |
# | 03-02-2025 16hs | Neutral | Bearish | Neutral | Neutral |
# | 03-02-2025 17hs | Neutral | Bearish | Neutral | Neutral |
# | 03-02-2025 18hs | Neutral | Bearish | Neutral | Neutral |
# | 03-02-2025 19hs | Neutral | Bearish | Neutral | Neutral |
# | 03-02-2025 20hs | Neutral | Bearish | Neutral | Neutral |
# | 03-02-2025 21hs | Neutral | Bearish | Neutral | Bullish |
# | 03-02-2025 22hs | Neutral | Bearish | Neutral | Bullish |
# | 03-02-2025 23hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 00hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 01hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 02hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 03hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 04hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 05hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 06hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 07hs | Neutral | Bearish | Bearish | Neutral |
# | 04-02-2025 08hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 09hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 10hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 11hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 12hs | Neutral | Bearish | Bullish | Neutral |
# | 04-02-2025 13hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 14hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 15hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 16hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 17hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 18hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 19hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 20hs | Neutral | Bearish | Bearish | Neutral |
# | 04-02-2025 21hs | Neutral | Bearish | Neutral | Bearish |
# | 04-02-2025 22hs | Neutral | Bearish | Neutral | Neutral |
# | 04-02-2025 23hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 00hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 01hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 02hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 03hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 04hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 05hs | Neutral | Bearish | Bullish | Neutral |
# | 05-02-2025 06hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 07hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 08hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 09hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 10hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 11hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 12hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 13hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 14hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 15hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 16hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 17hs | Neutral | Bearish | Bearish | Neutral |
# | 05-02-2025 18hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 19hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 20hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 21hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 22hs | Neutral | Bearish | Neutral | Neutral |
# | 05-02-2025 23hs | Neutral | Bearish | Bullish | Neutral |
# | 06-02-2025 00hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 01hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 02hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 03hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 04hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 05hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 06hs | Neutral | Bullish | Neutral | Bullish |
# | 06-02-2025 07hs | Neutral | Bullish | Neutral | Neutral |
# | 06-02-2025 08hs | Neutral | Bullish | Neutral | Neutral |
# | 06-02-2025 09hs | Neutral | Bullish | Neutral | Neutral |
# | 06-02-2025 10hs | Neutral | Bullish | Neutral | Neutral |
# | 06-02-2025 11hs | Neutral | Bullish | Neutral | Neutral |
# | 06-02-2025 12hs | Neutral | Bullish | Bearish | Neutral |
# | 06-02-2025 13hs | Neutral | Bullish | Neutral | Neutral |
# | 06-02-2025 14hs | Neutral | Bullish | Neutral | Neutral |
# | 06-02-2025 15hs | Neutral | Bearish | Neutral | Bearish |
# | 06-02-2025 16hs | Neutral | Bearish | Neutral | Bearish |
# | 06-02-2025 17hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 18hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 19hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 20hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 21hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 22hs | Neutral | Bearish | Neutral | Neutral |
# | 06-02-2025 23hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 00hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 01hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 02hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 03hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 04hs | Neutral | Bearish | Bullish | Neutral |
# | 07-02-2025 05hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 06hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 07hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 08hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 09hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 10hs | Neutral | Bearish | Neutral | Bullish |
# | 07-02-2025 11hs | Neutral | Bearish | Neutral | Bullish |
# | 07-02-2025 12hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 13hs | Neutral | Bearish | Neutral | Bullish |
# | 07-02-2025 14hs | Neutral | Bearish | Neutral | Bullish |
# | 07-02-2025 15hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 16hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 17hs | Neutral | Bearish | Bearish | Neutral |
# | 07-02-2025 18hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 19hs | Neutral | Bearish | Neutral | Neutral |
# | 07-02-2025 20hs | Bullish | Bearish | Neutral | Bearish |
# | 07-02-2025 21hs | Bullish | Bearish | Neutral | Bearish |
# | 07-02-2025 22hs | Bullish | Bearish | Neutral | Bearish |
# | 07-02-2025 23hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 00hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 01hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 02hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 03hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 04hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 05hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 06hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 07hs | Neutral | Bearish | Bullish | Neutral |
# | 08-02-2025 08hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 09hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 10hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 11hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 12hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 13hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 14hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 15hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 16hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 17hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 18hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 19hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 20hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 21hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 22hs | Neutral | Bearish | Neutral | Neutral |
# | 08-02-2025 23hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 00hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 01hs | Neutral | Bearish | Neutral | Bullish |
# | 09-02-2025 02hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 03hs | Neutral | Bearish | Neutral | Bullish |
# | 09-02-2025 04hs | Neutral | Bearish | Neutral | Bullish |
# | 09-02-2025 05hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 06hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 07hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 08hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 09hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 10hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 11hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 12hs | Neutral | Bearish | Neutral | Neutral |
# | 09-02-2025 13hs | Neutral | Bearish | Bearish | Neutral |
# | Date            | RSI     | EMA     | MACD    | BB      |
# +-----------------+---------+---------+---------+---------+
# ethusdt - 1H
# ------------------------------------------------------------------------------------------
```

### Entry

Computes stop_loss and take_profit price for asset and expected profit and potential loss

```bash
lt entry --help

# Usage: lt entry <MARKET> [STOP_LOSS] [TAKE_PROFIT] [AMOUNT]

# Arguments:
#   <MARKET>       
#   [STOP_LOSS]    
#   [TAKE_PROFIT]  
#   [AMOUNT]       

# Options:
#   -h, --help  Print help
```

### Indicators

Explains indicators

```bash
lt indicators --help

# Explain indicators

# Usage: lt indicators

# Options:
#   -h, --help  Print help
```

### Config

Defaults configs

```bash
lt config --help

# Configuration

# Usage: lt config [COMMAND]

# Commands:
#   show    Show current configuration
#   set     Set a configuration value
#   add     Add a configuration value
#   remove  Delete a configuration value
#   help    Print this message or the help of the given subcommand(s)

# Options:
#   -h, --help  Print help
```
