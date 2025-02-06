use std::thread::panicking;

// from pretty_num library, but modified

const SUFFIXES: [char; 7] = ['k', 'M', 'B', 'T', 'Q', 'q', 's'];

pub trait Int64Helper {
    fn pretty_format(self) -> String;
}

impl<N: Into<i64>> Int64Helper for N {
    fn pretty_format(self) -> String {
        let number: i64 = self.into();

        if number.abs() < 1000 {
            number.to_string()
        } else {
            let sign: i8 = if number < 0 { -1 } else { 1 };
            let mut number_as_float = number.abs() as f64;
            for suffix in SUFFIXES {
                number_as_float /= 1000f64;

                if number_as_float < 1000f64 {
                    return format!(
                        "{:.*}{suffix}",
                        if (number_as_float - number_as_float.floor()) < 0.1
                            || number_as_float >= 100f64
                        {
                            0
                        } else {
                            1
                        },
                        sign as f64 * number_as_float
                    );
                }
            }

            panic!("Number too large");
        }
    }
}