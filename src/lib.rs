use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

const GAME_WIDTH: f64 = 600.0;
const GAME_HEIGHT: f64 = 600.0;
const PLAYER_INITIAL_SPEED: f64 = 1.0;
 
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub speed: f64,
}

#[wasm_bindgen]
pub struct Game {
    pub width: f64,
    pub height: f64,
    pub player: Player,
}


#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, width: f64, height: f64, speed: f64) -> Player {
        Player { x, y, width, height, speed }
    }

    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.fill_rect(self.x, self.y, self.width, self.height);
        ctx.stroke();
        ctx.fill();
    }
    pub fn update(&mut self, keyevent: &str) {
        let _ = match keyevent {
            "Up" => self.y = (self.y - self.speed).max(self.height),
            "Right" => self.x = (self.x + self.speed).min(GAME_WIDTH),
            "Down" => self.y = (self.y + self.speed).min(GAME_HEIGHT - 100.0),
            "Left" => self.x = (self.x - self.speed).max(self.width),
            _ => (),
        };
    }
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, player: Player) -> Game {
        Game { width, height, player }
    }
}

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
    canvas.set_tab_index(0);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    set_common_style(&ctx);
    //
    
    let mut game = Game::new( GAME_WIDTH, GAME_HEIGHT, 
                              Player::new((GAME_WIDTH - 25.0)/2.0, GAME_HEIGHT-100.0, 50.0, 45.0, PLAYER_INITIAL_SPEED));

    wasm_bindgen_futures::spawn_local(async move {
        //console::log_1(&"Draw Rect".into());
        //let mut frame = -1.0;
        let interval_callback = Closure::wrap(Box::new(move || {
            //input: simulation
            //frame = (frame + 1.0) % 10.0;
            //let frame_name = format!("Frame #{}", frame + 1.0);

            // update:
            game.player.update("Up"); 
            // draw
            ctx.clear_rect(0.0, 0.0, game.width, game.height); //ctx.clear_rect(0.0, 0.0, 600.0, 600.0);
            game.player.draw(&ctx); 
            //console::log_1(&frame_name.into());
        }) as Box<dyn FnMut()>);

        window.set_interval_with_callback_and_timeout_and_arguments_0(
            interval_callback.as_ref().unchecked_ref(),
            50,
        );

        interval_callback.forget();
    });//^-- spawn_local

    Ok(())
}//^--fn main_js


fn set_common_style(ctx: &web_sys::CanvasRenderingContext2d) {
  ctx.set_shadow_color("#d53");
  ctx.set_shadow_blur(20.0);
  ctx.set_line_join("bevel");
  ctx.set_line_width(5.0);
}
