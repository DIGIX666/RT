use super::tuple::Tuple;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub canvas: Vec<Vec<Tuple>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            canvas: vec![vec![Tuple::default_color(); width]; height],
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
        self.canvas[y][x] = color;
    }
    
    pub fn to_ppm(&self) {
        print!("P3\n{} {}\n255\n", self.width, self.height);
        for line in self.canvas.clone().into_iter() {
            for pixel in line {
                print!("{}", pixel.clamp().as_str())
            }
        }
    }
}