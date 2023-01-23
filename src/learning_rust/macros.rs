use std::any::Any;

use paste::paste;

#[macro_use]
macro_rules! items {
    ( $($x:expr),* ) => {
        let mut num = 1;
        $(
            println!("{} is item number {}", $x, num);
            num += 1;
        )*
    };
}

macro_rules! create_struct {
    ($struct_name:ident, $(($field_name:ident: $field_type:ty)), *) => {
        #[derive(Default)]
        struct $struct_name {
            $(
                $field_name: $field_type,
            )*
        }
        impl $struct_name {
            fn new($($field_name: $field_type), *) -> $struct_name {
                $struct_name {
                    $(
                        $field_name: $field_name,
                    )*
                }
            }

            paste! {
                $(
                    fn [<get_$field_name>](&self) -> &$field_type {
                        &self.$field_name
                    }

                    fn [<set_$field_name>](&mut self, $field_name:$field_type) {
                        self.$field_name = $field_name;
                    }
                )*
                fn to_string(&self) -> String {
                    let mut string = format!("{} = [", stringify!($struct_name));
                    $(
                        string += format!("({} = {}), ",stringify!($field_name), self.[<get_$field_name>]()).as_str();
                    )*
                    string[..string.len()-2].to_string() + "]"
                }
            }
        }
    };
}

pub fn run() {
    // items![2, 5, 7, "hello"];
    create_struct!(stru, (field1: i32), (field2: String), (field3: f64));
    let mut stru:stru = Default::default();
    stru.set_field2("hello".to_string());
    println!("{}", stru.field1)
}