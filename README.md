# Rust Nannou Codealong for Schotter 

Learning Rust by doodling around with Nannou.
Starting Point [Schotter Repo](https://github.com/sidwellr/schotter)

![Alt text](schotter1/001.png?raw=true "Schotter1")

#### Noob Gotchas

I first forgot to add adding 'resolver=2' to the top level Cargo.toml.
I hadn't needed it for other nannou projects, so why do I need it here?
In the other projects I didn't have a workspace. So instead I had 
written 'edition = "2021"' in my Cargo.toml.

According to [The Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2021/default-cargo-resolver.html)
 edition = "2021" implies resolver = "2" in Cargo.toml.


