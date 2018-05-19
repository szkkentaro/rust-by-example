#[derive(Debug)]
struct A; // Concrete type `A`
struct S(A); // Concrete type `S`
struct SGen<T>(T); // Generic type `SGen`

// This is not generic function
fn reg_fn(s: S) {
    println!("The first field of S: {:?}", s.0);
}

// Type parameter is `A`, `A` has not been specified as a generic type, so this is aloso not generic.
fn gen_spec_t(s: SGen<A>) {
    println!("The first field of SGen: {:?}", s.0);
}

// This is not generic func , because i32 is not a generic type
fn gen_spec_i32(s: SGen<i32>) {
    println!("The first field of SGen: {:?}", s.0);
}

// This function is generic over `T`
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    // Explicitly specified type parameter `char` to `generic()`
    generic::<char>(SGen('A'));

    // Implicitly specified type parameter `char` to `generic()`
    generic(SGen('c'));
}
