use proc_macro_example::Count;

#[derive(Count)]
struct BasicStruct {
    a: i32,
}

fn main() {
    assert_eq!(BasicStruct::field_count(), 1);
}
