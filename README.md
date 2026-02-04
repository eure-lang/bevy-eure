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
