use wasm_bindgen::prelude::*;
pub mod string_convert;
pub mod turtle_draw;
pub mod types;

#[wasm_bindgen]
pub fn generate_fractal(
    axiom: String,
    string_convert_m: String,
    string_convert_l: String,
    string_convert_f: String,
    no_steps: u8,
    draw_start_x: f64,
    draw_start_y: f64,
    draw_start_angle: f64,
    angle_shift: f64,
) -> String {
    let rules = types::StringConvert {
        m: &string_convert_m,
        l: &string_convert_l,
        f: &string_convert_f,
    };

    let instruction_string = string_convert::recursion(&axiom, &rules, no_steps);

    return turtle_draw::apply_turtle(
        instruction_string,
        draw_start_x,
        draw_start_y,
        draw_start_angle,
        angle_shift,
    );
}
