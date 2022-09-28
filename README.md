# monero-vanity - WORK IN PROGRESS
monero-vanity is a CLI tool that generates vanity addresses (and spend keys) for Monero, like this one:
```
44hintoFpuo3ugKfcqJvh5BmrsTRpnTasJmetKC4VXCt6QDtbHVuixdTtsm6Ptp7Y8haXnJ6j8Gj2dra8CKy5ewz7Vi9CYW
```

## Comparison
| Generator                                                           | Hardware needed        | Regex | Calculates seed | Normal speed    | Regex speed |
|---------------------------------------------------------------------|------------------------|-------|-----------------|-----------------|-------------|
| [vanity-monero](https://github.com/monero-ecosystem/vanity-monero)  | CPU (x86, 32/64-bit)   | Yes   | Yes             | 400k/sec        | 170k/sec
| **[monero-vanity](https://github.com/hinto-janaiyo/monero-vanity)** | CPU (x86, 64-bit)      | Yes   | No              | 1.38million/sec | 1.38million/sec
| [vanity-xmr-cuda](https://github.com/SChernykh/vanity_xmr_cuda)     | NVIDIA GPU (with CUDA) | No    | No              | 8.1million/sec  |

*Tested with: Ryzen 5950x, GTX 1660 Ti*

## Usage
```
./monero-vanity
```
**Then enter:**
- Amount of CPUs (threads) to use
- Which type to look for (third/first/full)
- Which address pattern to look for

After finding the private spend key, enter it into:
```
./monero-wallet-cli --generate-from-spend-key
```
The generated wallet will have the address found.

**Note:**
- `I`, `O`, `l` are invalid characters in Monero addresses
- All characters must be ASCII or a regex pattern
- Third: matches 3rd-43rd character
- First: matches 1st-43rd character
- Full: matches 1st-95th character
- All are roughly the same speed
- [All can use Rust regex rules](https://docs.rs/regex/latest/regex)

**Example:**
- Third: `48hinto...` would match if `hinto` was typed
- First: `48hinto.../44h1nto...` would match if `^4(4|8)h(i|1)nto.*$` was typed
- Full: `48aaahinto...` would match if `^4.....hinto.*$` was typed

## Build
Normal:
```
cargo build --release
```
Optimized (1%~5% speed increase):
```
cargo build --profile optimized
```
Optimized for your specific CPU (15%~ speed increase, depending on your CPU):
```
RUSTFLAGS="-C target-cpu=native" cargo build --profile optimized
```

Add `--target x86_64-pc-windows-gnu` to build for Windows.

**Crate dependencies:**
- [rand](https://docs.rs/rand)
- [regex](https://docs.rs/regex)
- [monero](https://docs.rs/monero)
- [num_cpus](https://docs.rs/num_cpus)
- [base58-monero](https://docs.rs/base58-monero)
- [curve25519-dalek](https://docs.rs/curve25519-dalek)
