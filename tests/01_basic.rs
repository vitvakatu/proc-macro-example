use proc_macro_example::Count;

#[derive(Count)]
struct BasicStruct {
    a: i32,
}

#[derive(Count)]
enum BasicEnum {
    VariantA,
}

fn main() {
    assert_eq!(BasicStruct::field_count(), 1);
    assert_eq!(BasicEnum::field_count(), 1);
}
