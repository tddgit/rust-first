#![allow(dead_code)]
#![allow(unused_variables)]


use std::io::stdin;
use std::mem;

mod pm;

// const MEANING_OF_LIFE: u16 = 456; // no fixed address

fn main() {
    // primitive_types ();
    // operators();
    // scope_and_shadowing();
    // println!("const MEANING_OF_LIFE = {}", MEANING_OF_LIFE)
    // if_statement();
    // while_and_loop();
    // match_statecment();
    // for_loop();
    // combination_lock();
    // structures();
    // enums();
    // unions();
    // process_value();
    // option_T()
    // array();
    // slices();
    // tuples();
// pm::pattern_matching();
//     generics();
    vectors();
}

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
    a.push(44);
    println!("a = {:?}", a);
    //usize isize
    let idx: usize = 2;

    println!("a[2] = {}", a[idx]);

    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }
}

fn generics() {
    struct Point<T, V> {
        x: T,
        y: V,
    }
    struct Point1<T> {
        x: T,
        y: T,
    }
    struct Line<T> {
        start: Point1<T>,
        end: Point1<T>,
    }

    let a: Point<u16, i32> = Point { x: 0, y: 0 };
    let b: Point<f64, f32> = Point { x: 1.2, y: 3.4 };
    let c: Point<i32, f64> = Point { x: 3, y: 5.0 };
    let d: Point<i32, f64> = Point { x: 1, y: 4.5 };
    let x: Point1<f64> = Point1 { x: 1f64, y: 2f64 };
    let y: Point1<f64> = Point1 { x: 3f64, y: 4f64 };


    let myline = Line { start: x, end: y };
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    let (sum, product) = sp;

    println!("sp = {:?}", (sum, product));
    println!("{0} + {1} = {2}", x, y, sum);
    println!("{0} + {1} = {2}", x, y, product);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{1:?} , {2:?}, {0:?} ", combined.0, combined.1, combined);
    println!("{1:?} , {2:?}, {0:?} ", combined.0, (combined.0).0, (combined
        .0).1);

    let ((c, d), (e, f)) = combined;
    println!("{},{},{},{}", c, d, e, f);

    let foo = (true, 42.0, -1i8);

    println!("{:?}", foo);

    let meaning = 42;
    println!("{:?}", meaning);
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn use_slices(slice: &mut [i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4444;
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5];
    use_slices(&mut data[1..4]);
    use_slices(&mut data);
    println!("{:?}", data)
}

fn array() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5, ];

    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a has {} elements, first is {}", a.len(), a[0]);

    println!("{:?}", a);

    if a == [321, 2, 3, 4, 5] {
        println!("match");
    }

    let b = [1u64; 10];

    for i in 0..b.len() {
        println!("{}", b[i])
    };

    println!("b took up {} bytes", mem::size_of_val(&b));
    println!("b  {:?}", b);

    let mtx: [[f64; 3]; 2] = [
        [0.1, 0.2, 0.3],
        [0.4, 0.5, 0.6]
    ];

    println!("mtx = {:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                print!("diagonal: {} ", mtx[i][j]);
            }
        }
    }
    println!();
}


union IntOrFloat {
    i: i32,
    f: f32,
}

fn option_t() {
    let x = 3.0;
    let y = 1.0;

    //Option
    let result =
        if y != 0.0 { Some(x / y) } else { None };

    match result {
        Some(z) => {
            println!("{}/{} ={}", x, y, z)
        }
        None => println!("cannot divide by zero")
    }
    if let Some(z) = result {
        println!("result = {}", z)
    }
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value 42", );
            }
            IntOrFloat { f } => {
                println!("value = {}", f)
            }
        }
    }
}

fn unions() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFloat { i: 5 })
}

fn enums() {
    enum Color {
        Red,
        Green,
        Blue,
        RgbColor(u8, u8, u8),
        //tuple
        Cmyk { cyan: u8, magenta: u8, yellow: u8, black: u8 }, //struct
    }

    let c: Color = Color::Cmyk { cyan: 0, magenta: 128, yellow: 0, black: 0 };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("color: black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        Color::Cmyk { cyan: _, magenta: _, yellow: _, black: 255 } =>
            println!("black"),
        Color::Cmyk { cyan: a, magenta: b, yellow: c, black: d } => println!("cmyk({},{},{},{})", a, b,
                                                                             c, d),
    }
}

fn structures() {
    struct Point {
        x: f64,
        y: f64,
    }
    let p = Point { x: 34.5, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 3.0, y: 4.0 };
    struct Line {
        start: Point,
        end: Point,
    }
    let myline = Line { start: p, end: p2 };
}

enum State {
    Locked,
    Failed,
    Unlocked,
}

fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();
    println!(" string = {}, code = {}", entry, code);
    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => entry.push_str(&input.trim_end()),
                    Err(_) => continue,
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    state = State::Failed
                }
            }
            State::Failed => {
                println!("Failed");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("Unlocked");
                return;
            }
        }
    }
}

fn match_statement() {
    let country_code = 44;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "unknown",
        _ => "invalid",
    };

    println!("the country code {} is {}", country_code, country)
}

fn for_loop() {
    for x in 1..11 {
        if x == 3 {
            continue;
        }
        if x == 8 {
            break;
        }
        println!("x = {}", x)
    }

    for (pos, y) in (30..42).enumerate() {
        println!("{} : {}", pos, y)
    }
}

fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }

        println!("x = {}", x)
    }

    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 {
            break;
        }
    }
}

fn if_statement() {
    let temp = 25;
    if temp > 30 {
        println!("really hot outside")
    } else if temp < 10 {
        println!("really cold!")
    } else {
        println!("temperature is OK")
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };

    println!("today is {}", day);

    println!(
        "is it {}",
        if temp > 20 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );

    println!(
        "it is {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    )
}

fn scope_and_shadowing() {
    let a = 123;
    println!("a = {}", a);
    let a = 777;
    println!("a = {}", a);

    {
        let a = 888;
        let b = 456;
        println!("a = {}, b = {}", a, b);
    }
}

fn operators() {
    //arithmetic operators

    let mut a = 2 + 3 * 4;
    println!("{}", a);

    a += 1;
    a -= 2;

    println!("remainder of {}/{} = {}", a, 3, (a % 3));

    // let mut a_cubed = i16::pow(a, 3);
    // let mut a_cubed = i32::pow( 4);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    println!("b = {}", b);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}", b, b_cubed);
    println!("{} pied = {}", b, b_to_pi);

    //bitwise rotate
    let c = 1 | 2;
    println!("1 | 2 = {}", c);

    let two_to_10 = 1 << 10;
    println!("2^10 =  {}", two_to_10);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    let x = 5;
    let x_is_5 = x == 5;
}

fn primitive_types() {
    let a: u8 = 123;
    let b: i8 = -123;

    // println!("a = {}, b ={}", a, b);

    // a = 432;
    // b = 567;
    // b = 122;
    let mut c = 123456789; // 32-bit signed integer
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    c = -1;
    println!("c = {} after modification", c);

    let z: isize = 123456789;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, size = {}, {}-bit os", z, size_of_z, size_of_z * 8);

    let d: char = 'x';
    println!("d = {}, size = {}", d, mem::size_of_val(&d));

    let e = 2.5; // double precision value, 8 bytes or 64 bits, f 64
    println!("e = {}, size = {}", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, size = {}", g, mem::size_of_val(&g));
}
