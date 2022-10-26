use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::ImageData;

#[wasm_bindgen]
pub fn paint(
    max_iter: u32,
    center_x: f64,
    center_y: f64,
    scale: f64,
    super_sample_factor: u32,
    color_step: u32,
    color_number: u32,
    color_shift: u32,
) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("stage")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let super_sample_canvas = document
        .get_element_by_id("supersample")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let super_sample_context = super_sample_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    super_sample_canvas.set_width(super_sample_factor * canvas.width());
    super_sample_canvas.set_height(super_sample_factor * canvas.height());
    let stage = Stage {
        center: (center_x, center_y),
        scale: scale * super_sample_factor as f64,
        width: super_sample_canvas.width(),
        height: super_sample_canvas.height(),
        color_step,
        color_number,
        color_shift,
    };
    let mandelbrot = Mandelbrot::new(max_iter, stage);
    let data = mandelbrot.gen_image_data();
    super_sample_context.put_image_data(&data, 0., 0.).unwrap();
}

#[wasm_bindgen]
pub fn transfer(dx: f64, dy: f64, dw: f64, dh: f64) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("stage")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let super_sample_canvas = document
        .get_element_by_id("supersample")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
    context
        .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &super_sample_canvas,
            0.,
            0.,
            super_sample_canvas.width() as f64,
            super_sample_canvas.height() as f64,
            dx as f64,
            dy as f64,
            dw as f64,
            dh as f64,
        )
        .unwrap();
}

#[derive(Copy, Clone)]
pub struct Stage {
    center: (f64, f64),
    scale: f64,
    width: u32,
    height: u32,
    color_step: u32,
    color_number: u32,
    color_shift: u32,
}

impl Stage {
    pub fn new(
        center_x: f64,
        center_y: f64,
        scale: f64,
        width: u32,
        height: u32,
        color_step: u32,
        color_number: u32,
        color_shift: u32,
    ) -> Stage {
        Stage {
            center: (center_x, center_y),
            scale,
            width,
            height,
            color_step,
            color_number,
            color_shift,
        }
    }
}

impl Stage {
    pub fn xmin_ymax_xdif_ydif(&self) -> (f64, f64, f64, f64) {
        let (xmin, ymin, xmax, ymax) = (
            self.center.0 - self.width as f64 / self.scale / 2., // xmin
            self.center.1 - self.height as f64 / self.scale / 2., // ymin
            self.center.0 + self.width as f64 / self.scale / 2., // xmax
            self.center.1 + self.height as f64 / self.scale / 2., // ymax
        );
        (
            xmin,
            ymax,
            (xmax - xmin) / self.width as f64,
            (ymax - ymin) / self.height as f64,
        )
    }
}

pub struct Mandelbrot {
    pub max_iter: u32,
    pub stage: Stage,
}

impl Mandelbrot {
    fn new(max_iter: u32, stage: Stage) -> Mandelbrot {
        Mandelbrot { max_iter, stage }
    }
    fn escape_time(&self, c: &ComplexNumber) -> u32 {
        let mut iter_count: u32 = 1;
        let mut z = c.to_owned();
        while (iter_count < self.max_iter) && (z.modulus_square() <= 4.) {
            z = z.square().add(c);
            iter_count += 1;
        }
        iter_count
    }
    fn escape_time_iter(&'_ self) -> Box<dyn Iterator<Item = u32> + '_> {
        let w = 0..self.stage.width;
        let h = 0..self.stage.height;
        let (xmin, ymax, xdif, ydif) = self.stage.xmin_ymax_xdif_ydif();
        let data_iterator = h
            .into_iter()
            .map(move |y| w.clone().into_iter().map(move |x| (x, y)))
            .flatten()
            .map(move |(x, y)| {
                let xx = xmin + x as f64 * xdif;
                let yy = ymax - y as f64 * ydif;
                let c = ComplexNumber::new(xx, yy);
                self.escape_time(&c)
            });
        Box::new(data_iterator)
    }
    fn gen_image_data(&self) -> ImageData {
        let mut data = self.escape_time_iter();
        let mut data: Vec<u8> = data.as_mut().map(|n| self.gen_color(n)).flatten().collect();
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut data),
            self.stage.width,
            self.stage.height,
        )
        .unwrap();
        data
    }
    fn gen_color(&self, n: u32) -> [u8; 4] {
        let shifted_n = self.stage.color_shift * self.stage.color_step + n;
        Mandelbrot::hsva_to_rgba((
            (shifted_n / self.stage.color_step % self.stage.color_number) as f32
                / self.stage.color_number as f32
                * 360.,
            // (shifted_n % self.stage.color_step + self.stage.color_step * 2) as f32 / (self.stage.color_step * 3) as f32,
            1.,
            if n == self.max_iter {
                0.
            } else {
                (shifted_n % self.stage.color_step + self.stage.color_step) as f32
                    / (self.stage.color_step * 2) as f32
            },
            255u8,
        ))
    }

    fn hsva_to_rgba(hsv: (f32, f32, f32, u8)) -> [u8; 4] {
        let h = hsv.0;
        let s = hsv.1;
        let v = hsv.2;
        let a = hsv.3;
        let c = v * s;
        let x = c * (1. - ((h / 60.) % 2. - 1.).abs());
        let m = v - c;
        let cm = ((c + m) * 255.) as u8;
        let xm = ((x + m) * 255.) as u8;
        let mm = (m * 255.) as u8;
        match h {
            _ if h < 60. => [cm, xm, mm, a],
            _ if h < 120. => [xm, cm, mm, a],
            _ if h < 180. => [mm, cm, xm, a],
            _ if h < 240. => [mm, xm, cm, a],
            _ if h < 300. => [xm, mm, cm, a],
            _ if h < 360. => [cm, mm, xm, a],
            _ => [0, 0, 0, a],
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct ComplexNumber {
    real: f64,
    imaginary: f64,
}

impl ComplexNumber {
    pub fn new(real: f64, imaginary: f64) -> ComplexNumber {
        ComplexNumber { real, imaginary }
    }
    pub fn add(mut self, number: &Self) -> ComplexNumber {
        self.real += number.real;
        self.imaginary += number.imaginary;
        self
    }
    pub fn square(mut self) -> ComplexNumber {
        (self.real, self.imaginary) = (
            self.real * self.real - self.imaginary * self.imaginary,
            2. * self.real * self.imaginary,
        );
        self
    }
    pub fn modulus_square(&self) -> f64 {
        self.real * self.real + self.imaginary * self.imaginary
    }
}

#[cfg(test)]
pub mod test {
    use crate::{ComplexNumber, Mandelbrot, Stage};
    use wasm_bindgen_test::wasm_bindgen_test;
    #[wasm_bindgen_test]
    pub fn complex_add() {
        let a = ComplexNumber::new(1., 1.);
        let b = ComplexNumber::new(1., 1.);
        let c = ComplexNumber::new(2., 2.);
        assert_eq!(a.add(&b), c);
    }
    #[wasm_bindgen_test]
    pub fn complex_square() {
        let a = ComplexNumber::new(4., 4.);
        let b = ComplexNumber::new(0., 32.);
        assert_eq!(a.square(), b);
    }
    #[wasm_bindgen_test]
    pub fn complex_modulus() {
        let a = ComplexNumber::new(3., 4.);
        assert_eq!(a.modulus_square(), 25.);
    }
    #[wasm_bindgen_test]
    pub fn mandelbrot_escape_time() {
        let stage = Stage {
            center: (0., 0.),
            scale: 100.,
            width: 100,
            height: 100,
            color_step: 0,
            color_number: 0,
            color_shift: 0,
        };
        let a = Mandelbrot::new(100, stage);
        let b = a.escape_time(&ComplexNumber::new(0., 0.));
        assert_eq!(b, 100);
    }
    #[wasm_bindgen_test]
    pub fn gen() {
        let stage = Stage {
            center: (0., 0.),
            scale: 100.,
            width: 100,
            height: 100,
            color_step: 0,
            color_number: 0,
            color_shift: 0,
        };
        let a = Mandelbrot::new(100, stage);
        let mut b = a.escape_time_iter();
        let c: Vec<u32> = b.as_mut().into_iter().filter(|x| *x > 1u32).collect();
        panic!("{:?}", c);
    }
}
