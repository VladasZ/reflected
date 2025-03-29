#[derive(Default, Debug)]
enum Vethost {
    #[default]
    Strong,
    _Weak,
}

#[test]
fn reflect_enum() {
    dbg!(Vethost::default());
}
