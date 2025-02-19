trait Render {
    fn render(&self, text: &str);
}

struct VectorRenderer;

impl Render for VectorRenderer {
    fn render(&self, text: &str) {
        println!("Rendering {} as lines", text);
    }
}

struct RasterRenderer;

impl Render for RasterRenderer {
    fn render(&self, text: &str) {
        println!("Rendering {} as pixels", text);
    }
}

struct Circle<'a> {
    renderer: &'a dyn Render,
    x: f64,
    y: f64,
    radius: f64,
}

impl<'a> Circle<'a> {
    fn new(renderer: &'a dyn Render, x: f64, y: f64, radius: f64) -> Self {
        Self {
            renderer,
            x,
            y,
            radius,
        }
    }

    fn draw(&self) {
        self.renderer.render(&format!(
            "Circle at ({}, {}) with radius {}",
            self.x, self.y, self.radius
        ));
    }
}

struct Square<'a> {
    renderer: &'a dyn Render,
    x: f64,
}

impl<'a> Square<'a> {
    fn new(renderer: &'a dyn Render, x: f64) -> Self {
        Self { renderer, x }
    }

    fn draw(&self) {
        self.renderer.render(&format!("Square at ({})", self.x));
    }
}

fn main() {
    let vector_renderer = VectorRenderer;
    let raster_renderer = RasterRenderer;

    let circle = Circle::new(&vector_renderer, 10.0, 10.0, 5.0);
    let square = Square::new(&vector_renderer, 10.0);

    circle.draw();
    square.draw();

    let circle = Circle::new(&raster_renderer, 10.0, 10.0, 5.0);
    let square = Square::new(&raster_renderer, 10.0);

    circle.draw();
    square.draw();
}
