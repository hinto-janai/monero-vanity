<div align="center">

# monero-vanity
monero-vanity is a GUI/CLI tool that generates vanity addresses for Monero, like this one:
</div>

```
44hintoFpuo3ugKfcqJvh5BmrsTRpnTasJmetKC4VXCt6QDtbHVuixdTtsm6Ptp7Y8haXnJ6j8Gj2dra8CKy5ewz7Vi9CYW
```

<div align="center">

#### GUI

https://user-images.githubusercontent.com/101352116/229327381-2b001b8a-b1a0-4889-93d2-ce6a9c2a858f.mp4

#### CLI

https://user-images.githubusercontent.com/101352116/229327380-52a7f8a2-fcf7-4eb0-9ec8-c0554f86a2bc.mp4

</div>

---

* [Comparison](#Comparison)
* [Estimate](#Estimate)
* [GUI Usage](#GUI-Usage)
* [CLI Usage](#CLI-Usage)
* [Install](#Install)
* [Implementation](#Implementation)
* [Build](#Build)

---

## Comparison
| Generator                                                           | Hardware needed        | Regex | Calculates seed | Normal speed    | Regex speed |
|---------------------------------------------------------------------|------------------------|-------|-----------------|-----------------|-------------|
| [vanity-monero](https://github.com/monero-ecosystem/vanity-monero)  | CPU (x86, 32/64-bit)   | Yes   | Yes             | 400k/sec        | 170k/sec
| **[monero-vanity](https://github.com/hinto-janai/monero-vanity)** | CPU (x86, 64-bit)      | Yes   | No              | 5.8million/sec  | 5.8million/sec
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

## GUI Usage
<div align="center">

After finding an address, create a new Monero wallet:

<img src="images/1.png" width="50%"/>

Input the address, private view key, and private spend key:

<img src="images/2.png" width="50%"/>

And verify your new wallet address is correct:

<img src="images/3.png" width="50%"/>

</div>

## CLI Usage
```bash
Usage: monero-vanity [--OPTIONS]

Options:
  -t, --threads <THREADS>  How many threads to use [default: HALF_THREADS]
  -p, --pattern <PATTERN>  Address regex pattern to look for
  -f, --first              Start from 1st character instead of: ^4.PATTERN.*$
  -r, --refresh <REFRESH>  How many milliseconds in-between output refreshes [default: 500]
  -v, --version            Print version
  -h, --help               Print help (see more with '--help')
```
Example 1 - Basic pattern using half threads:
```bash
./monero-vanity --pattern hinto

> 44hinto...
```

Example 2 - Advanced regex pattern using half threads:
```bash
./monero-vanity --first --pattern "^4(4|8)h(i|1)nto.*$"

> 48hinto...
```

After finding the private spend key:
```
./monero-wallet-cli --generate-from-spend-key YOUR_WALLET_NAME
```
Enter the private key and the generated wallet will have the address found.

**Notes:**
- [Rust regex is allowed in any mode](https://docs.rs/regex/latest/regex/#syntax)
- All characters must be ASCII, Unicode, or a regex pattern
- `I`, `O`, `l`, `0`, `+`, `/` are invalid characters in [Monero addresses](https://monerodocs.org/cryptography/base58)
- Using slightly less than max threads might be faster

`monero-vanity` automatically prefixes your input with `^4.` and suffixes it with `.*$` so that your PATTERN starts from the 3rd character until the 43rd character of the address.

Example input: `hinto`  
Actual regex used: `^4.hinto.*$`

To disable this, use `--first`.

Warning: this puts you in full control of the regex, you can input any value, even an impossible one.

## Install
Download [here.](https://github.com/hinto-janai/monero-vanity/releases)

### Cargo
If you have `cargo`, you can install with:
```bash
cargo install monero-vanity
```

### Arch
If you're using Arch Linux, you can install from the [AUR](https://aur.archlinux.org/packages/monero-vanity-bin) with:
```bash
paru monero-vanity
```

## Implementation
1. [Random `[u8; 64]` is generated (512 bits/64 bytes)](https://github.com/hinto-janai/monero-vanity/blob/28e902a830049a7478d15be94fd3c55488c1aac6/src/address.rs#L46)
2. [Scalar is created by reducing the above bytes](https://github.com/hinto-janai/monero-vanity/blob/28e902a830049a7478d15be94fd3c55488c1aac6/src/address.rs#L67)
3. [Compressed EdwardsPoint's base58 encoding is checked (4...) with regex](https://github.com/hinto-janai/monero-vanity/blob/28e902a830049a7478d15be94fd3c55488c1aac6/src/address.rs#L79)
4. [If match, create full address and return to user, else...](https://github.com/hinto-janai/monero-vanity/blob/28e902a830049a7478d15be94fd3c55488c1aac6/src/address.rs#L83)
5. [Increment EdwardsPoint by 1 and continue](https://github.com/hinto-janai/monero-vanity/blob/28e902a830049a7478d15be94fd3c55488c1aac6/src/address.rs#L101)

**Notes:**
- [Each thread seeds its own RNG](https://github.com/hinto-janai/monero-vanity/blob/28e902a830049a7478d15be94fd3c55488c1aac6/src/address.rs#L64)
- The (optional) private _view_ key is also created by reducing [512 random bits](https://github.com/hinto-janai/monero-vanity/blob/28e902a830049a7478d15be94fd3c55488c1aac6/src/address.rs#L89)

## Build
```
cargo build --release
```
Optimized for your specific CPU (up to 15%~ speed increase):
```
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Thanks
Big thanks to [kayabaNerve](https://github.com/kayabaNerve) for helping me with ECC cryptography and Rust (he's the reason why it's fast).
