# desc2addr

desc2addr is a simple cli tool used for generate receive/change addresses from a Liana descriptor.

# Build

Install Rust toolchain ([see here](https://www.rust-lang.org/tools/install))

and run this in this repo:

```shell
cargo build --release
```

the binary will be in target/release

## Usage
``` 
Usage: desc2addr [OPTIONS] --descriptor <DESCRIPTOR>

Options:
  -d, --descriptor <DESCRIPTOR>  The descriptor to generate address from
  -s, --start <START>            Starting index
  -c, --count <COUNT>            Address count
  -h, --help                     Print help
```


## Example

```shell
$ desc2addr -d "wsh(or_d(pk([d4ab66f1/48'/1'/0'/2']tpubDEXYN145WM4rVKtcWpySBYiVQ229pmrnyAGJT14BBh2QJr7ABJswchDicZfFaauLyXhDad1nCoCZQEwAW87JPotP93ykC9WJvoASnBjYBxW/<0;1>/*),and_v(v:pkh([33fa0ffc/48'/1'/0'/2']tpubDEqzYAym2MnGqKdqu2ZtGQkDTSrvDWCrcoamspjRJR78nr8w5tAgu371r8LtcyWWWXGemenTMxmoLhQM3ww8gUfobBXUWxLEkfR7kGjD6jC/<0;1>/*),older(65535))))#r5c5gqy8" -s 2 -c 2
[
    {
        "index": 2,
        "receive": "tb1qsxwhjg2uue523wp5j4ysql86msexvmtxca655klkqq6w6rz0jytqtd42nu",
        "change": "tb1qk8yam0x25fxttck0kzpdsrwhsw23mfxf7gdhmhn9sa23al4elu9sye6aah"
    },
    {
        "index": 3,
        "receive": "tb1qm25c3nztw6z0zp6mpq05380z4s5fql7q7rcs7rxufu4js4z67stq59nt22",
        "change": "tb1qulgz2k6zjj8m4nxa2hlkzsyvemh6kacgzr3k2p0qz7en0033vw5qkvm25q"
    }
]
```
