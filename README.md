<p align="center">
  <i>ldv-rs</i> is a rust library designed for interacting with the `.ldv` file format.
</p>

> [!WARNING]
> this crate is completely fanmade and has no affiliation with the luduvo devs.

> [!IMPORTANT]
> this library is in a pre-1.0.0 state! expect breaking changes between versions.

---

## features

> [!TIP]
> most users will want to import the prelude, via `luduvo_rs::prelude::*`

- user profile data (search by id, one result)
- user friends data (search by id, multiple results)
- user querying (search by username, multiple results)
- places data (search by name, multiple results)

## quick start

```rust
use ldv_rs::{data_types::Vec3, dom::Dom, file::File};

fn main() -> std::io::Result<()> {
    let data = std::fs::read("assets/in/world.ldv")?;
    let file = File::from(&data).unwrap();

    let mut dom = Dom::from_file(&file);

    dom.create_entity(999);
    dom.set_position(999, Vec3 { x: 1.0, y: 2.0, z: 3.0 });

    let new_file = dom.to_file().unwrap();
    let bytes = new_file.to_bytes();

    std::fs::write("assets/out/world.ldv", bytes)?;

    println!("wrote to assets/out/world.ldv");

    Ok(())
}
```

## contributors

> [!NOTE]
> this crate is MIT-licensed. feel free to do whatever with it! all contributions (pull requests, issues) are welcomed, including to the docs.

- [Eeviika](https://github.com/Eeviika) for [#1](https://github.com/luduvo-devhub/luduvo-rs/pull/1) (Small changes)

## need help?

- contact me on discord! my discord username is `@primiti_ve`.
    - my preferred method of communication is joining the [luduvo development hub](https://discord.gg/FcjTvuWKRk)! it's full of like-minded developers who will gladly help you out with any issues.
- [create an issue](https://github.com/luduvo-devhub/luduvo-rs/issues)! this is better for organisation purposes, although you should also join the luduvo development hub aswell.
