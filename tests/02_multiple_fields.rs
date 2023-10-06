use proc_macro_example::Count;

#[derive(Count)]
struct MultipleFieldsStruct {
    a: i32,
    b: String,
    c: bool,
}

fn main() {
    assert_eq!(MultipleFieldsStruct::field_count(), 3);
}
