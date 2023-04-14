# lai.rs

cute rust bindings for the [lightweight aml interpreter](https://github.com/managarm/lai) (LAI).

as an aml interpreter, lai is used by avian to provide aml support for the [acpi](https://en.wikipedia.org/wiki/ACPI) standard.

## usage

```rust
struct LaiHost;

impl lai::Host for LaiHost {
    // host functions...
}

let lai_host = Arc::new(LaiHost);
lai::init(lai_host);

// after this, the host has been successfully initialized.
```
