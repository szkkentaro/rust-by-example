struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}
// T is generic param, U is caller
impl<T, U> DoubleDrop<T> for U {
    // This method takes owenership of both passed arguments,
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // Deallocate `empty` and `null`
    empty.double_drop(null);

    // empty;
    // null;
}
