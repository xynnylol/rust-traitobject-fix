# traitobject

> Unsafe helpers for dealing with raw trait objects.

## Patching the dead original crate

Seeing the original crate has not been updated in over 8 years the version has not been changed from 0.1.0

Reason this exists is to [fix a warning that WILL BECOME A HARD ERROR](https://github.com/rust-lang/rust/issues/56484) in a future version of the rust compiler.

You can add the patched crate using this line in your cargo.toml

```
[patch.crates-io]
traitobject = { git = 'https://github.com/xynnylol/rust-traitobject-fix.git' }
```

## Author

[Jonathan Reem](https://medium.com/@jreem) is the primary author and maintainer
of traitobject.

## License

MIT/Apache-2.0
