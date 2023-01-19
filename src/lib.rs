use qrcode::render::svg;
use qrcode::QrCode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn qrcodegen(data: &str) -> String {
    let code = QrCode::new(data).unwrap();
    let image = code.render::<svg::Color>().build();

    image.into()
}
