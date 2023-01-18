/// A function rarely modifying the data
mod test_1 {
    use std::borrow::Cow;

    fn remove_whitespaces(s: &str) -> Cow<str> {
        if s.contains(' ') {
            Cow::Owned(s.to_string().replace(' ', ""))
        } else {
            Cow::Borrowed(s)
        }
    }
    
    pub fn run() {
        let value = remove_whitespaces("Hello world!");
        println!("{}", value);
    }
}

/// A struct optionally owning the data
mod test_2 {
    use std::borrow::Cow;

    struct User<'a> {
        first_name: Cow<'a, str>,
        last_name: Cow<'a, str>,
    }

    impl<'a> User<'a> {

        pub fn new_owned(first_name: String, last_name: String) -> User<'static> {
            User {
                first_name: Cow::Owned(first_name),
                last_name: Cow::Owned(last_name),
            }
        }
    
        pub fn new_borrowed(first_name: &'a str, last_name: &'a str) -> Self {
            Self {
                first_name: Cow::Borrowed(first_name),
                last_name: Cow::Borrowed(last_name),
            }
        }
    
    
        pub fn first_name(&self) -> &str {
            &self.first_name
        }

        pub fn last_name(&self) -> &str {
            &self.last_name
        }
    }

    pub fn run() {
        // Static lifetime as it owns the data
        let user: User<'static> = User::new_owned("James".to_owned(), "Bond".to_owned());
        println!("Name (1): {} {}", user.first_name(), user.last_name());

        // Static lifetime as it borrows 'static data
        let user: User<'static> = User::new_borrowed("James", "Bond");
        println!("Name (2): {} {}", user.first_name, user.last_name);

        let first_name = "James".to_owned();
        let last_name = "Bond".to_owned();

        // Non-static lifetime as it borrows the data
        let user= User::new_borrowed(&first_name, &last_name);
        println!("Name (3): {} {}", user.first_name, user.last_name);
    }
}

/// A clone on write struct
mod test_3 {
    use std::borrow::Cow;

    struct LazyBuffer<'a> {
        data: Cow<'a, [u8]>,
    }
    
    impl<'a> LazyBuffer<'a> {
    
        pub fn new(data: &'a[u8]) -> Self {
            Self {
                data: Cow::Borrowed(data),
            }
        }
    
        pub fn data(&self) -> &[u8] {
            &self.data
        }
    
        pub fn append(&mut self, data: &[u8]) {
            self.data.to_mut().extend(data)
        }
    }

    pub fn run() {
        let data = vec![0u8; 10];

        // No memory copied yet
        let mut buffer = LazyBuffer::new(&data);
        println!("Data(1): {:?}", buffer.data());

        // The data is cloned
        buffer.append(&[1, 2, 3]);
        println!("Data(2): {:?}", buffer.data());

        // The data is not cloned on further attempts
        buffer.append(&[4, 5, 6]);
        println!("Data(3): {:?}", buffer.data());
    }
}

mod test_4 {
    use std::borrow::{Borrow, Cow};
    use std::ops::Deref;

    #[derive(Debug)]
    struct MyString {
        data: String
    }

    impl Borrow<MyStr> for MyString {
        fn borrow(&self) -> &MyStr {
            unsafe { &*(self.data.as_str() as *const str as *const MyStr) }
        }
    }

    #[derive(Debug)]
    #[repr(transparent)]
    struct MyStr {
        data: str,
    }

    impl ToOwned for MyStr {
        type Owned = MyString;
    
        fn to_owned(&self) -> MyString {
            MyString {
                data: self.data.to_owned()
            }
        }
    }
}

fn main() {
    test_1::run();
    test_2::run();
    test_3::run();
}
