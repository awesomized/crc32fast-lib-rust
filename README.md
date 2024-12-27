# crc32fast-lib

Fast, SIMD-accelerated [CRC-32/ISO-HDLC](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iso-hdlc) (aka `crc32`) checksum computation in Rust exposed as a C-compatible shared library.

Results in a dramatic performance improvement. For example, when [using it via FFI in PHP](https://github.com/awesomized/crc-fast-php), it's >10X faster than PHP's native [crc32](https://www.php.net/crc32) implementation.

## Changes

See the [change log](CHANGELOG.md).

## Usage

`cargo build` will produce a shared library target (`.so` on Linx, `.dll` on Windows, `.dylib` on macOS, etc) and a C header file.

Use the header file and library as you would normally when using a shared library in your language of choice.

## References

- [crc32fast](https://github.com/srijs/rust-crc32fast) - The underlying Rust library which implemented this SIMD-accelerated approach.
- [crc-fast-php](https://github.com/awesomized/crc-fast-php) - An implementation of this library in PHP using FFI.
- [crc64fast-nvme](https://github.com/awesomized/crc64fast-nvme) - A similar project which computes [CRC-64/NVME](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme) checksums at >20GiB/s.