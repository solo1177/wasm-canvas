use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

#[wasm_bindgen]
pub fn apply_grayscale(image_data: ImageData) -> ImageData {
    let width = image_data.width();
    let height = image_data.height();
    let mut data = image_data.data();

    for i in (0..data.len()).step_by(4) {
        let r = data[i] as f32 * 0.3;
        let g = data[i + 1] as f32 * 0.59;
        let b = data[i + 2] as f32 * 0.11;
        let gray = (r + g + b) as u8;
        data[i] = gray;
        data[i + 1] = gray;
        data[i + 2] = gray;
    }

    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width, height).unwrap()
}
