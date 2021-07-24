struct TupleStruct(i32, i32);
struct NormalStruct {
    a: i32,
    b: i32,
}

fn main() {
    let ns = NormalStruct { a: 1, b: 2 };
    let ts = TupleStruct(1, 2);

    // shortcut to initialize the fields of a struct using the values of the
    // fields of another struct
    let ns2 = NormalStruct { a: 5, ..ns };
    let ts2 = TupleStruct { 0: 1, ..ts }; // for TupleStruct it needs curly brackets
                                          // and implicit field names

    println!("Accessing Normal Struct by name - {}, {}", ns.a, ns.b);
    println!("accessing Tuple Struct by name - {}, {}", ts.0, ts.1);
}
