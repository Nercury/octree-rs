pub struct Layout {
    pub items: Vec<Item>,
}

impl Layout {
    pub fn new() -> Layout {
        Layout { items: Vec::new() }
    }

    pub fn with(mut self, index: u32, format: Format, padding: Padding) -> Self {
        self.items.push(Item {
            index: index,
            format: format,
            padding: padding
        });

        self
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Item {
    pub index: u32,
    pub format: Format,
    pub padding: Padding,
}

impl Item {
    pub fn bytes(&self) -> i32 {
        self.format.bytes() + self.padding.bytes()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum Padding {
    p0,

    p8,
    p8_p8,
    p8_p8_p8,

    p16,
    p16_p16,
    p16_p16_p16,

    p32,
    p32_p32,
    p32_p32_p32,

    p64,
    p64_p64,
    p64_p64_p64,
}

impl Padding {
    pub fn bytes(&self) -> i32 {
        use self::Padding::*;

        match *self {
            p0 => 0,

            p8 => 1,
            p8_p8 => 2,
            p8_p8_p8 => 3,

            p16 => 2,
            p16_p16 => 4,
            p16_p16_p16 => 6,

            p32 => 4,
            p32_p32 => 8,
            p32_p32_p32 => 12,

            p64 => 8,
            p64_p64 => 16,
            p64_p64_p64 => 24,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum Format {
    i8,
    i8_i8,
    i8_i8_i8,
    i8_i8_i8_i8,

    i8_float,
    i8_i8_float,
    i8_i8_i8_float,
    i8_i8_i8_i8_float,

    i16,
    i16_i16,
    i16_i16_i16,
    i16_i16_i16_i16,

    i16_float,
    i16_i16_float,
    i16_i16_i16_float,
    i16_i16_i16_i16_float,

    i32,
    i32_i32,
    i32_i32_i32,
    i32_i32_i32_i32,

    i32_float,
    i32_i32_float,
    i32_i32_i32_float,
    i32_i32_i32_i32_float,

    u8,
    u8_u8,
    u8_u8_u8,
    u8_u8_u8_u8,

    u8_float,
    u8_u8_float,
    u8_u8_u8_float,
    u8_u8_u8_u8_float,

    u16,
    u16_u16,
    u16_u16_u16,
    u16_u16_u16_u16,

    u16_float,
    u16_u16_float,
    u16_u16_u16_float,
    u16_u16_u16_u16_float,

    u32,
    u32_u32,
    u32_u32_u32,
    u32_u32_u32_u32,

    u32_float,
    u32_u32_float,
    u32_u32_u32_float,
    u32_u32_u32_u32_float,

    f16,
    f16_f16,
    f16_f16_f16,
    f16_f16_f16_f16,

    f32,
    f32_f32,
    f32_f32_f32,
    f32_f32_f32_f32,

    f64,
    f64_f64,
    f64_f64_f64,
    f64_f64_f64_f64,

    i2_i10_i10_i10_rev,
    u2_u10_u10_u10_rev,
    u10_u11_u11_rev,

    i2_i10_i10_i10_rev_float,
    u2_u10_u10_u10_rev_float,
    u10_u11_u11_rev_float,
}

impl Format {
    pub fn bytes(&self) -> i32 {
        use self::Format::*;

        match *self {
            i8 => 1,
            i8_i8 => 2,
            i8_i8_i8 => 3,
            i8_i8_i8_i8 => 4,

            i8_float => 1,
            i8_i8_float => 2,
            i8_i8_i8_float => 3,
            i8_i8_i8_i8_float => 4,

            i16 => 2,
            i16_i16 => 4,
            i16_i16_i16 => 6,
            i16_i16_i16_i16 => 8,

            i16_float => 2,
            i16_i16_float => 4,
            i16_i16_i16_float => 6,
            i16_i16_i16_i16_float => 8,

            i32 => 4,
            i32_i32 => 8,
            i32_i32_i32 => 12,
            i32_i32_i32_i32 => 16,

            i32_float => 4,
            i32_i32_float => 8,
            i32_i32_i32_float => 12,
            i32_i32_i32_i32_float => 16,

            u8 => 1,
            u8_u8 => 2,
            u8_u8_u8 => 3,
            u8_u8_u8_u8 => 4,

            u8_float => 1,
            u8_u8_float => 2,
            u8_u8_u8_float => 3,
            u8_u8_u8_u8_float => 4,

            u16 => 2,
            u16_u16 => 4,
            u16_u16_u16 => 6,
            u16_u16_u16_u16 => 8,

            u16_float => 2,
            u16_u16_float => 4,
            u16_u16_u16_float => 6,
            u16_u16_u16_u16_float => 8,

            u32 => 4,
            u32_u32 => 8,
            u32_u32_u32 => 12,
            u32_u32_u32_u32 => 16,

            u32_float => 4,
            u32_u32_float => 8,
            u32_u32_u32_float => 12,
            u32_u32_u32_u32_float => 16,

            f16 => 2,
            f16_f16 => 4,
            f16_f16_f16 => 6,
            f16_f16_f16_f16 => 8,

            f32 => 4,
            f32_f32 => 8,
            f32_f32_f32 => 12,
            f32_f32_f32_f32 => 16,

            f64 => 8,
            f64_f64 => 16,
            f64_f64_f64 => 24,
            f64_f64_f64_f64 => 32,

            i2_i10_i10_i10_rev => 4,
            u2_u10_u10_u10_rev => 4,
            u10_u11_u11_rev => 4,

            i2_i10_i10_i10_rev_float => 4,
            u2_u10_u10_u10_rev_float => 4,
            u10_u11_u11_rev_float => 4,
        }
    }
}