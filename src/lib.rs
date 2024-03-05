use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

const GAME_WIDTH: f64 = 600.0;
const GAME_HEIGHT: f64 = 600.0;
const PLAYER_INITIAL_SPEED: f64 = 5.5;


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
#[derive(Clone, Copy)]
pub struct Game {
    pub width: f64,
    pub height: f64,
    pub player: Player,
    pub keyevent: i32,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, width: f64, height: f64, speed: f64) -> Player {
        Player {
            x,
            y,
            width,
            height,
            speed,
        }
    }

    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
//        ctx.fill_rect(self.x, self.y, self.width, self.height);
//        ctx.stroke();
//        ctx.fill();

        ctx.set_stroke_style(&"yellow".into()); // &JsValue::from_str("yellow")
        ctx.set_fill_style(&"black".into()); // into know to use JsValue::from_str
        ctx.stroke_rect(self.x, self.y, self.width, self.height);
        ctx.fill_rect(self.x, self.y, self.width, self.height);
    }
    pub fn update(&mut self, keyevent: i32) {
        let _ = match keyevent {
            0 => self.y = (self.y - self.speed).max(self.height), //"ArrowUp"
            1 => self.x = (self.x + self.speed).min(GAME_WIDTH - (self.width * 2.0)), //"ArrowRight"
            2 => self.y = (self.y + self.speed).min(GAME_HEIGHT - (self.height * 2.0)), //"ArrowDown"
            3 => self.x = (self.x - self.speed).max(self.width), //"ArrowLeft"
            4 => console::log_1(&"spacebar to shoot".into()), 
            _ => (), // do nothing 
        };
    }
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, player: Player) -> Game {
        Game { width, height, player, keyevent: 6 }
    }
    pub fn get_keyevent(&self) -> i32 {
        self.keyevent
    }
    pub fn set_keyevent(&mut self, keyevent: i32) {
        self.keyevent = keyevent;
    }
    pub fn update(&mut self) {
        console::log_1(&format!("Game: {}", self.keyevent).into());
        self.player.update(self.keyevent);
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
    let _ = canvas.focus();
    // canvas.set_tab_index(0); not working and looks to be a hack: 

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let _ = canvas.focus();
    set_common_style(&ctx);

    let game = Rc::new( RefCell::new( Game::new(
                            GAME_WIDTH,
                            GAME_HEIGHT,
                            Player::new(
                                (GAME_WIDTH - 25.0) / 2.0,
                                GAME_HEIGHT - 100.0,
                                50.0,
                                45.0,
                                PLAYER_INITIAL_SPEED,
                            ),
                        ))
    );

    wasm_bindgen_futures::spawn_local(async move {
        let game = game.clone();
        
        // input
        handle_input(&canvas, Rc::clone(&game)); 

        let interval_callback = Closure::wrap(Box::new({
            let game = game.clone();            
            move || {
                // update:
                game.borrow_mut().update();
                // draw
                clear_screen(&ctx, &canvas);
                game.borrow().player.draw(&ctx);
            }
        }) as Box<dyn FnMut()>);

        let _ = window
            .set_interval_with_callback_and_timeout_and_arguments_0(interval_callback
                                                                        .as_ref()
                                                                        .unchecked_ref(), 
                                                                        50);

        interval_callback.forget();
    }); //^-- spawn_local

    Ok(())
} //^--fn main_js

fn handle_input(canvas: &web_sys::HtmlCanvasElement, game: Rc<RefCell<Game>>) {
    let game_up = game.clone();
    let game_down = game.clone();
    let gamekeys: HashMap<String, i32> = HashMap::from([ ("ArrowUp".to_string(), 0), ("ArrowRight".to_string(),1) ,("ArrowDown".to_string(), 2),("ArrowLeft".to_string(), 3),(" ".to_string(), 4)]);  
 
    let onkeydown = Closure::wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        console::log_1(&keycode.key().into());
        let k = *gamekeys.get(&keycode.key()).unwrap_or(&5);
        game_down.borrow_mut().set_keyevent(k); //5 is do nothing
 
        console::log_1(&format!("key {}", game_down.borrow().get_keyevent()).into());
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);


    let onkeyup =
        Closure::wrap(Box::new(move |_keycode: web_sys::KeyboardEvent| {
        game_up.borrow_mut().set_keyevent(5); //5 is do nothing
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    canvas.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));

    canvas.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));

    onkeydown.forget();
    onkeyup.forget();
}

fn clear_screen(ctx: &web_sys::CanvasRenderingContext2d, canvas: &web_sys::HtmlCanvasElement) {
    //NOPE: ctx.fill_style("black");
    ctx.set_fill_style(&"rgb(0,0,0)".into());
    ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

}
fn set_common_style(ctx: &web_sys::CanvasRenderingContext2d) {
    ctx.set_shadow_color("#d53");
    ctx.set_shadow_blur(20.0);
    ctx.set_line_join("bevel");
    ctx.set_line_width(5.0);
}
