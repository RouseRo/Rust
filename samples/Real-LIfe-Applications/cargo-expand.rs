macro_rules! sum_macro {

    ($($x:expr),*) => {

        {

            let mut sum = 0;

            $(sum += $x;)*

            sum

        }

    };

}



fn main() {

    let result = sum_macro!(1, 2, 3, 4, 5);

}