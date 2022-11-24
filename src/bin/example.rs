use proc_macro_example::Count;

#[derive(Count)]
enum Example {
    Variant1,
    Variant2,
    Variant3,
}

fn main() {
    println!("Our enum has {} variants", Example::count());
}
