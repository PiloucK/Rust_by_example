fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let a_suffix_annotation_float = 1.0f64;
    let an_integer = 5i32;
    let a_regular_annotation_int: i32 = 5;

    // currently a f64
    let default_float = 3.0;
    // currently an i32 both like in c
    let default_integer = 7;

    // this is at compile time considered as i64 from context
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    // mut to be able to change its value
    let mut mutable = 12;
    mutable = 21;

    // mutable = true; impossible as the type cannot change

    // principle of shadowing allows overwittings even with type change
    let mutable = true;
}
