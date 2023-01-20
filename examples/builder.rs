use color_eyre::Result;
use nidhogg_derive::Builder;

#[derive(Default, Builder)]
struct Test {
    foo: i32,
    bar: Vec<u8>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let baz = Test::builder().bar(vec![10]).foo(12).build();

    println!("foo: {}, bar: {:?}", baz.foo, baz.bar);
    Ok(())
}
