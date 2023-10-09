use proc_macro_example::Count;

#[derive(Count)]
struct MultipleFieldsStruct {
    a: i32,
    b: String,
    c: bool,
}

#[derive(Count)]
enum MultipleVariantsEnum {
    VariantA(i32),
    VariantB(String),
    VariantC(bool),
}

fn main() {
    assert_eq!(MultipleFieldsStruct::field_count(), 3);
    assert_eq!(MultipleVariantsEnum::field_count(), 3);
}
