mod painter;
use painter::Painter;

use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen(js_name = Painter)]
pub struct PainterWrapper {
    painter: Painter
}

#[wasm_bindgen(js_class = Painter)]
impl PainterWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PainterWrapper {
        PainterWrapper { 
            painter: Painter::new(1000, 1000) 
        }
    }

    pub fn get_pixel(&self, i: u32, j: u32) -> Array {
        let pixel = self.painter.get_pixel(i, j);
        let res = Array::new_with_length(3);
        res.set(0, JsValue::from(pixel.0[0]));
        res.set(1, JsValue::from(pixel.0[1]));
        res.set(2, JsValue::from(pixel.0[2]));
        res
    }
}
