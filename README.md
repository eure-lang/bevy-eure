# bevy-eure

Bevy asset loader for Eure format.

## Example

```rust
App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(EureAssetPlugin::<Script>::new(&["script.eure"]))
```

```rust
#[derive(FromEure, IntoEure)]
// #[eure(crate = "bevy_eure::eure::document")] This needed is you don't add `eure` to the dependency
pub struct User {
	  name: String,
		#[eure(via = "bevy_eure::Vec3Proxy")]
		position: Vec3,
}
```

## Versioning

| bevy-eure | bevy   | eure  |
|-----------|--------|-------|
| 0.1.0     | 0.18.0 | 0.1.6 |

## Development

You need to use nightly compiler to run tests since this crate use the nightly feature `variant_count` for asserting non_exhaustive enums matches to the remote definition of enums on bevy.

```bash
cargo +nightly test
```
