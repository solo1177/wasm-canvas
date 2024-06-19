// use wasm_bindgen::prelude::*;
// use wasm_bindgen::Clamped;
// use web_sys::ImageData;

// #[wasm_bindgen]
// pub struct CanvasState {
//     width: u32,
//     height: u32,
//     data: Vec<u8>,
// }

// #[wasm_bindgen]
// impl CanvasState {
//     #[wasm_bindgen(constructor)]
//     pub fn new(width: u32, height: u32) -> CanvasState {
//         let data = vec![255; (width * height * 4) as usize]; // Initialize with white background
//         CanvasState { width, height, data }
//     }

//     pub fn generate_image_data(&self) -> ImageData {
//         ImageData::new_with_u8_clamped_array_and_sh(Clamped(&self.data), self.width, self.height).unwrap()
//     }

//     pub fn draw_circle(&mut self, cx: u32, cy: u32, radius: u32) {
//         for y in 0..self.height {
//             for x in 0..self.width {
//                 let dx = cx as i32 - x as i32;
//                 let dy = cy as i32 - y as i32;
//                 if dx * dx + dy * dy <= (radius as i32) * (radius as i32) {
//                     let offset = (y * self.width + x) * 4;
//                     self.data[offset as usize] = 255;       // R
//                     self.data[(offset + 1) as usize] = 0;   // G
//                     self.data[(offset + 2) as usize] = 0;   // B
//                     self.data[(offset + 3) as usize] = 255; // A
//                 }
//             }
//         }
//     }
// }
