# ppmd-rs
Library that implements different versions of PPMD algorithm (compress and decompress) 

### Dependencies
* [Rust](https://www.rust-lang.org/) 1.58 or newer
* Cargo

### How to build
* Clone this repository
* `cargo build --release`

### Implementation details
Initially, the library was ported from C version of PPMD, based on this implementation: https://github.com/svpv/ppmd-mini , then with C2Rust it was converted to Rust and hacked a little.

### Feedback
If you have any suggestions or want to report a bug - feel free to create an issue in this repo. Thank you!

### Useful links

* [Wiki](https://en.wikipedia.org/wiki/Prediction_by_partial_matching) about the algorithm
* [C++ PPMD](https://github.com/byronknoll/cmix/blob/master/src/models/ppmd.cpp) implementation
* [C PPMD](https://github.com/svpv/ppmd-mini) implementation (implemented only 7-Zip "I" variant)
