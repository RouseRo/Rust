#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Blue,
    Silver
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E)
}

#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T)
}


fn create_car_colour_blue() -> CarColour {
    let my_car_colour: CarColour = CarColour::Blue;
    my_car_colour
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not under 5!".to_string())
    }
}

fn check_under_five_bullt_in(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err("Not under 5!".to_string())
    }
}


fn remainder_zero(num_check: f32) -> GivenOption<f32>{
    let remainder: f32 = num_check % 10.0;
        if remainder != 0.0 {
        GivenOption::Some(remainder)  
        } else {
            GivenOption::None
        }
}
fn remainder_zero_bult_in(num_check: f32) -> Option<f32>{
    let remainder: f32 = num_check % 10.0;
        if remainder != 0.0 {
            Some(remainder)  
        } else {
            None
        }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_colour = create_car_colour_blue();
        dbg!(car_colour);

        let is_under_five_res: GivenResult<u8, String> = check_under_five(2);
        dbg!(is_under_five_res);

        let is_under_five_res: GivenResult<u8, String> = check_under_five(7);
        dbg!(is_under_five_res);

        let is_under_five_res: Result<u8, String> = check_under_five_bullt_in(7);
        dbg!(is_under_five_res);

        let remainder: GivenOption<f32> = remainder_zero(10.0);
        dbg!(remainder);

        let remainder: Option<f32> = remainder_zero_bult_in(10.0);
        dbg!(remainder);

    }
}