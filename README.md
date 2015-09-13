ruby-mri-rs is intended to faciliate embedding the MRI Ruby VM inside Rust programs in an idiomatic way.
All work is based on reading ruby.h and [other documentation](http://silverhammermba.github.io/emberb/c/).

Files:
src/ffi.rs - direct low-level bindings to the C API.
src/lib.rs - high-level bindings

Pull requests and github issues welcome! This is a spare-time project, so if you find something that
doesn't work that you need, go ahead and fix it!
