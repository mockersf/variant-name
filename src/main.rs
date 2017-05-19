#[macro_use]
extern crate variant_name_derive;
extern crate variant_name_trait;

use variant_name_trait::VariantName;

#[derive(Debug, VariantName)]
enum Foo<Ta, Tb, Tc, Td> {
    A(Ta),
    B(Tb),
    C(Tc),
    D(Td),
}

fn main() {
    let a: Foo<i32, i32, i32, i32> = Foo::A(42);
    let b: Foo<i32, i32, i32, i32> = Foo::B(42);
    let c: Foo<i32, i32, i32, i32> = Foo::C(42);
    let d: Foo<i32, i32, i32, i32> = Foo::D(42);
    println!("{:?}: {}", a, a.variant_name());
    println!("{:?}: {}", b, b.variant_name());
    println!("{:?}: {}", c, c.variant_name());
    println!("{:?}: {}", d, d.variant_name());
}

