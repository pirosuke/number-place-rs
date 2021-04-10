# number place

A Number Place game using [Yew](https://yew.rs/docs/en/).

## Building
### Web View

```
cd src_yew
wasm-pack build --target web --out-name wasm --out-dir ./static
```

## Generate patterns

```
cargo run -p number-place-pattern-generator -- -o patterns -n 10
```

## Generate problems

```
cargo run -p number-place-problem-generator -- -p patterns -o src_yew/static/problems/
```
