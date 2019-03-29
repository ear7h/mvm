macro_rules! dense_enum {
    ($name:ident;
        $($var:ident) , * ,
    ) => {
        #[allow(dead_code)]
        #[derive(Debug)]
        pub enum $name {
            $($var) , *
        }

        impl $name {
            pub fn from_int(i:u8) -> $name {
                return unsafe { std::mem::transmute::<u8, $name>(i) }
            }

            pub fn to_str(&self) -> &'static str {
                match self {
                    $($name::$var => stringify!($var)) , *
                }
            }

            pub fn from_string(s: String) -> Result<$name, String> {
                match s.to_uppercase().as_str() {
                    $(stringify!($var) => Ok($name::$var)) , * ,
                    _ => Err(format!("{} not a[n] {}", s, stringify!($name))),
                }
            }
        }
    }
}
