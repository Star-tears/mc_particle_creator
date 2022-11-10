pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

pub const color_list: [Color; 17] = [
    //白色
    Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
    //七彩色
    Color {
        r: 0.8,
        g: 0.0,
        b: 0.0,
        a: 0.9,
    },
    Color {
        r: 0.8,
        g: 0.544,
        b: 0.0,
        a: 0.9,
    },
    Color {
        r: 0.8,
        g: 0.8,
        b: 0.0,
        a: 0.9,
    },
    Color {
        r: 0.0,
        g: 0.8,
        b: 0.0,
        a: 0.9,
    },
    Color {
        r: 0.0,
        g: 0.8,
        b: 0.8,
        a: 0.9,
    },
    Color {
        r: 0.0,
        g: 0.0,
        b: 0.8,
        a: 0.9,
    },
    Color {
        r: 0.442,
        g: 0.0,
        b: 0.8,
        a: 0.9,
    },
    // 亮rgb
    Color {
        r: 0.95,
        g: 0.45,
        b: 0.45,
        a: 0.9,
    },
    Color {
        r: 0.45,
        g: 0.95,
        b: 0.45,
        a: 0.9,
    },
    Color {
        r: 0.45,
        g: 0.45,
        b: 0.95,
        a: 0.9,
    },
    //渐变基色
    Color {
        r: 0.65,
        g: 0.55,
        b: 0.45,
        a: 0.9,
    },
    Color {
        r: 0.65,
        g: 0.45,
        b: 0.55,
        a: 0.9,
    },
    Color {
        r: 0.55,
        g: 0.65,
        b: 0.45,
        a: 0.9,
    },
    Color {
        r: 0.55,
        g: 0.45,
        b: 0.65,
        a: 0.9,
    },
    Color {
        r: 0.45,
        g: 0.65,
        b: 0.55,
        a: 0.9,
    },
    Color {
        r: 0.45,
        g: 0.55,
        b: 0.65,
        a: 0.9,
    },
];
