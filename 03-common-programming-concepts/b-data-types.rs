fn main() {
    let eight_bit:i8 = 10;               // n = 8 - 1 from -(2^n) to (2^n) - 1     => -128 - 127
    let unsigned_eight_bit:u8 = 10;      // n = 8 from 0 to 2^n - 1              => 0    - 256
    
    let sixteen_bit:i16 = 10;            // n = 16 - 1 from -(2^n) to 2^n - 1    => -32768 - 32767
    let unsigned_sixteen_bit:u16 = 10;   // n = 16 from 0 to 2^n - 1             => 0    - 65536

    let thirtytwo_bit:i32 = 10;          // n = 32 - 1 from -(2^n) to 2^n - 1
    let unsigned_thirtytwo_bit:u32 = 10; // n = 32 from 0 to 2^n - 1

    let sixtyfour_bit:i64 = 10;          // n = 64 - 1 from -(2^n) to 2^n - 1
    let unsigned_sixtyfour_bit:u64 = 10; // n = 64 from 0 to 2^n - 1

    let onetwoeight_bit:i128 = 10;       // n = 128 - 1 from -(2^n) to 2^n - 1
    let unsigned_onetwoeight_bit:u128 = 10; // n = 128 from 0 to 2^n - 1

    let literal: u16 = 10_000;

    //Floating 64 bits is default for rust
    let floating = 10.0; // f64
    let floating2:f32 = 10.0; // f32

    // Boolean is one bit size
    let t = true;
    let f: bool = false;

    // Char type, 4 bytes in size
    let letter = 'R'; // Implicitly a char
    let emoji: char = '😀' ; // Explicitly a char


    // COMPOUND TYPES
    let tuple_example: (i32, f64, u8) = (500, 6.4, 1); // cannot grow or shrink
    let five_hundred = tuple_example.0;

    let array_example = [1, 2, 3, 4, 5];
    let array_example2: [i32; 5] = [1, 2, 3, 4, 5]; // type:size
    let first_element = array_example[0];

}