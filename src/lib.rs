use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
   //
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    set_common_style(&ctx);
    //
    wasm_bindgen_futures::spawn_local(async move {
        console::log_1(&"Draw triangle".into());
        let mut frame = -1.0;
        let interval_callback = Closure::wrap(Box::new(move || {
            frame = (frame + 1.0) % 300.0;
            let frame_name = format!("Frame #{}", frame + 1.0);
            ctx.clear_rect(0.0, 0.0, 600.0, 600.0);
            draw_rect(&ctx, frame); 
            console::log_1(&frame_name.into());
        }) as Box<dyn FnMut()>);

        window.set_interval_with_callback_and_timeout_and_arguments_0(
            interval_callback.as_ref().unchecked_ref(),
            50,
        );

        interval_callback.forget();
    });//^-- spawn_local

    Ok(())
}//^--fn main_js

fn draw_rect(ctx: &web_sys::CanvasRenderingContext2d, y: f64) {
    ctx.fill_rect(10.0, y, 150.0, 100.0);
    //ctx.fill();
    ctx.stroke();
    ctx.fill();
}

fn set_common_style(ctx: &web_sys::CanvasRenderingContext2d) {
  ctx.set_shadow_color("#d53");
  ctx.set_shadow_blur(20.0);
  ctx.set_line_join("bevel");
  ctx.set_line_width(5.0);
}
