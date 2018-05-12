mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

fn main() {
    let open_box = my::OpenBox {
        contents: "public infomation",
    };
    println!("OpenBox contents is {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field name
    // let closed_box = ClosedBox {
    //     contents: "private infomation",
    // }

    // However, PUblic struct with private fields can be created using public constructor.
    let _closed_box = my::ClosedBox::new("private information");
}
