# monero-vanity
monero-vanity is a CLI tool that generates vanity addresses (and spend keys) for Monero, like this one:
```
44hintoFpuo3ugKfcqJvh5BmrsTRpnTasJmetKC4VXCt6QDtbHVuixdTtsm6Ptp7Y8haXnJ6j8Gj2dra8CKy5ewz7Vi9CYW
```

## Comparison
| Generator                                                           | Hardware needed        | Regex | Calculates seed | Normal speed    | Regex speed |
|---------------------------------------------------------------------|------------------------|-------|-----------------|-----------------|-------------|
| [vanity-monero](https://github.com/monero-ecosystem/vanity-monero)  | CPU (x86, 32/64-bit)   | Yes   | Yes             | 400k/sec        | 170k/sec
| **[monero-vanity](https://github.com/hinto-janaiyo/monero-vanity)** | CPU (x86, 64-bit)      | Yes   | No              | 5.8million/sec  | 5.8million/sec
| [vanity-xmr-cuda](https://github.com/SChernykh/vanity_xmr_cuda)     | NVIDIA GPU (with CUDA) | No    | No              | 8.1million/sec  |

*Tested with: Ryzen 5950x, GTX 1660 Ti*

## Estimate
| Characters | Example          | Rough Time Estimate  |
|------------|------------------|----------------------|
| 1          | `44h`            | Instant              |
| 2          | `44hi`           | Instant              |
| 3          | `44hin`          | Instant              |
| 4          | `44hint`         | 2 seconds            |
| 5          | `44hinto`        | 1 minute, 30 seconds |
| 6          | `44hintoj`       | 1 hour, 30 minutes   |
| 7          | `44hintoja`      | 4 days, 10 hours     |
| 8          | `44hintojan`     | 280 days             |
| 9          | `44hintojana`    | 49 years             |
| 10         | `44hintojanai`   | 3,151 years          |
| 11         | `44hintojanaiy`  | 100,852 years        |
| 12         | `44hintojanaiyo` | Pretty much never    |

*Assuming speed of 5.8million keys a second*

## Usage
```
./monero-vanity
```
**Then enter:**
1. Amount of CPUs (threads) to use
2. RNG safety level (Normal/Safe)
3. Which mode to use (Third/First/Full)
4. Which address pattern to look for

After finding the private spend key:
```
./monero-wallet-cli --generate-from-spend-key YOUR_WALLET_NAME
```
Enter the private key and the generated wallet will have the address found.

**Note:**
- [Rust regex is allowed in any mode](https://docs.rs/regex/latest/regex/#syntax)
- All characters must be ASCII, Unicode, or a regex pattern
- `I`, `O`, `l`, `0`, `+`, `/` are invalid characters in [Monero addresses](https://monerodocs.org/cryptography/base58)
- Using slightly less than max threads might be faster (see `Benchmark` below for more info)

**Safety:**
 - Normal: Private key is generated by reducing a 256-bit integer
 - Safe: Private key is generated by reducing a 512-bit integer

**Modes:**
- Third: matches 3rd-43rd character
- First: matches 1st-43rd character
- Full (6x SLOWER): matches 1st-95th character

**Example:**
- Third: `48hinto...` would match if `hinto` or `h.(N|n).o` was typed
- First: `48hinto.../44h1nto...` would match if `^4(4|8)h(i|1)nto.*$` was typed
- Full: `4...hinto` would match if `^.*hinto$` was typed

## Benchmark
For speed reasons, monero-vanity does lazy iteration counting (current thread tries * threads used). This can be inaccurate because threads can run at very different speeds. This benchmark mode will use an atomic counter, giving 100% accuracy. You can use this mode to more accurately gauge your CPUs performance using different amount of threads. Using slightly less than max threads may actually be faster (28 threads seems like the sweet spot for a Ryzen 5950x).

Start benchmark mode with this flag:
```
./monero-vanity --benchmark
```
**Then enter:**
1. How many threads to use
4. How many iterations

monero-vanity will look for an impossible address (`^..l.*$`) using 1-43th character + 256-bit RNG settings until all threads have reached the target iteration, and print some benchmark stats.

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
- [num-format](https://docs.rs/num-format)
- [base58-monero](https://docs.rs/base58-monero)
- [curve25519-dalek](https://docs.rs/curve25519-dalek)

## Thanks
Big thanks to [kayabaNerve](https://github.com/kayabaNerve) for helping me with ECC cryptography and Rust (he's the reason why it's fast).
