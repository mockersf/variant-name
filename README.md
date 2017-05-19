# Variant Name

How to use `proc-macro ` to add a function `variant_name`to enum to get the variant name without its parameters

## Examples

    #[derive(VariantName)]
    enum Foo<Ta, Tb> {
        A(i32),
        B,
        C(Ta, Tb),
        D { a: Ta, b: Tb }
    }


    let a: Foo<i32, u32> = Foo::A(42);
    let b: Foo<i32, u32> = Foo::B;
    let c: Foo<i32, u32> = Foo::C(42, 37);
    let d: Foo<i32, u32> = Foo::D { a: 42, b: 37 };
    assert_eq!("A", a.variant_name());
    assert_eq!("B", b.variant_name());
    assert_eq!("C", c.variant_name());
    assert_eq!("D", d.variant_name());