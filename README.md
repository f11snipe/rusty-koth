# koth

*King of the hill game service*

### Install

```bash
cargo install koth
```

### Examples

```bash
# Watch /root/king.txt and listen on all interfaces port 9999
koth --bind 0.0.0.0 --port 9999 --king-file /root/king.txt

# Override path to json data file
koth --data-file /tmp/data.json

# Custom scoring values (2/200ms => 10 points per second)
koth --tick-points 2 --tick-interval 200
```

### Usage

```
KOTH

Usage: koth [OPTIONS]

Options:
  -b, --bind <HOST>               Host to bind to (0.0.0.0, 127.0.0.1, etc) [default: 127.0.0.1]
  -p, --port <PORT>               Port to use [default: 9999]
  -d, --data-file <FILE>          Path to data file (json) [default: ./data.json]
  -k, --king-file <FILE>          File to monitor as king file [default: ./king.txt]
  -t, --tick-points <VALUE>       Amount of points per tick [default: 1]
  -i, --tick-interval <INTERVAL>  Interval for main score loop (in ms) [default: 500]
  -B, --no-banner                 Don't show the banner
  -v, --verbose                   Show details about interactions
  -h, --help                      Print help
  -V, --version                   Print version
```
