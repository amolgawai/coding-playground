fn main() {
    data_types_scalers();
    data_types_compound();
    conditionals();
    loops();

    println!("function returned {}", a_func(20, 10));
}

fn data_types_scalers() {
    println!("scaler data types ---------");
    let x_int_min = std::i32::MIN;
    let x_int_max = std::i32::MAX;

    println!(
        "Integers - default: i32 min -> {}, max -> {}",
        x_int_min, x_int_max
    );

    let u8_int_min = std::u8::MIN;
    let u8_int_max = std::u8::MAX;
    let i8_int_min = std::i8::MIN;
    let i8_int_max = std::i8::MAX;
    println!(
        "Integers - u8: min -> {}, max -> {}",
        u8_int_min, u8_int_max
    );
    println!(
        "Integers - i8: min -> {}, max -> {}",
        i8_int_min, i8_int_max
    );

    let u16_int_min = std::u16::MIN;
    let u16_int_max = std::u16::MAX;
    let i16_int_min = std::i16::MIN;
    let i16_int_max = std::i16::MAX;
    println!(
        "Integers - u16: min -> {}, max -> {}",
        u16_int_min, u16_int_max
    );
    println!(
        "Integers - i16: min -> {}, max -> {}",
        i16_int_min, i16_int_max
    );

    let u32_int_min = std::u32::MIN;
    let u32_int_max = std::u32::MAX;
    println!(
        "Integers - u32: min -> {}, max -> {}",
        u32_int_min, u32_int_max
    );

    let u64_int_min = std::u64::MIN;
    let u64_int_max = std::u64::MAX;
    let i64_int_min = std::i64::MIN;
    let i64_int_max = std::i64::MAX;
    println!(
        "Integers - u64: min -> {}, max -> {}",
        u64_int_min, u64_int_max
    );
    println!(
        "Integers - i64: min -> {}, max -> {}",
        i64_int_min, i64_int_max
    );

    let u128_int_min = std::u128::MIN;
    let u128_int_max = std::u128::MAX;
    let i128_int_min = std::i128::MIN;
    let i128_int_max = std::i128::MAX;
    println!(
        "Integers - u128: min -> {}, max -> {}",
        u128_int_min, u128_int_max
    );
    println!(
        "Integers - i128: min -> {}, max -> {}",
        i128_int_min, i128_int_max
    );

    // floating point values
    let f32_float_min = std::f32::MIN;
    let f32_float_max = std::f32::MAX;
    let f64_float_min = std::f64::MIN;
    let f64_float_max = std::f64::MAX;
    println!(
        "Floats - f32: min -> {:e}, max -> {:e}",
        f32_float_min, f32_float_max
    );
    println!(
        "Floats, default - f64: min -> {:e}, max -> {:e}",
        f64_float_min, f64_float_max
    );

    let a_char = 'a';
    let b_char = 'A';
    let a_str = "a string";
    println!("characters {}, {} and string {}", a_char, b_char, a_str);

    let truthy = true;
    let falsey = false;
    println!("Booleans - {} and {}", truthy, falsey);
}

fn data_types_compound() {
    println!("compound data types -----------");

    let a_tuple = (22, 55.6, 'z', "I am a tuple");
    println!("Tuple - {:?}", a_tuple);
    println!(
        "Tuple elements - {}, {}, {}, {}",
        a_tuple.0, a_tuple.1, a_tuple.2, a_tuple.3
    );

    let a_array = [2, 3, 4, 5];
    let b_array = [10.5; 5];
    let c_array: [char; 2] = ['a', 'b'];
    println!("Arrays - {:?}, {:?}, {:?}", a_array, b_array, c_array);
    println!("Array elements - {}, {}", c_array[0], c_array[1]);
}

fn conditionals() {
    println!("conditionals - if, if-else, if-else if-else -----------");
    let a_number = 15;

    if a_number < 20 {
        println!("{}, is less than 10", a_number);
    }

    if a_number == 18 {
        println!("Doesn't print anything");
    } else {
        println!("Print else block");
    }

    if a_number % 4 == 0 {
        println!("number is divisible by 4");
    } else if a_number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("{} is not divisible by 2 or 4", a_number);
    }

    let a_value = if a_number > 0 {
        -a_number
    } else {
        0 - a_number
    };
    println!("{} from conditional expression", a_value)
}

fn loops() {
    println!("Loops - loop, while, for ---------");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Ran loop {} times and got result {}", counter, result);

    while counter != 0 {
        println!("In while loop with counter = {}", counter);
        counter -= 2;
    }

    for an_item in (1..11).rev() {
        println!("Iterating in for loop with item {}", an_item);
    }
}

fn a_func(x: i32, y: i32) -> i32 {
    println!("Function with implicit and explicit return -------");
    if x + y > 50 {
        return x + y; //Explicit return
    }

    50 // Implicit return - last line without ; (expression)
}
