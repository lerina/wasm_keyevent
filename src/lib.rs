use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
//use lazy_static::lazy_static;
//use std::sync::Mutex;
//lazy_static! {
//    static ref MUTABLE_GLOBAL_VARIABLE: Mutex<String> = Mutex::new("_".to_string());
//}

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
#[derive(Clone, Copy)]
pub struct Game {
    pub width: f64,
    pub height: f64,
    pub player: Player,
    pub keyevent: Option<i32>,
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
        ctx.fill_rect(self.x, self.y, self.width, self.height);
        ctx.stroke();
        ctx.fill();
    }
    pub fn update(&mut self, keyevent: i32) {
        let _ = match keyevent {
            0 => self.y = (self.y - self.speed).max(self.height), //"ArrowUp"
            1 => self.x = (self.x + self.speed).min(GAME_WIDTH), //"ArrowRight"
            2 => self.y = (self.y + self.speed).min(GAME_HEIGHT - 100.0), //"ArrowDown"
            3 => self.x = (self.x - self.speed).max(self.width), //"ArrowLeft"
            4 => todo!(), //spacebar to shoot //" " 
            _ => (), // do nothing 
        };
    }
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, player: Player) -> Game {
        Game { width, height, player, keyevent: None }
    }
    pub fn get_keyevent(&self) -> Option<i32> {
        self.keyevent
    }
    pub fn set_keyevent(&mut self, keyevent: i32) {
        self.keyevent = Some(keyevent);
    }
    pub fn update(&mut self) {
        let _ = match self.keyevent {
            Some(x) => self.player.update(x),
            _ => {},
        };
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
        prepare_input(&canvas, *game.borrow()); 

        let interval_callback = Closure::wrap(Box::new({
            let game = game.clone();            
            move || {
            // input
            let user_action = game.borrow().get_keyevent(); 
            // update:
            game.borrow_mut().update();
            // draw
            ctx.clear_rect(0.0, 0.0, game.borrow().width, game.borrow().height); //ctx.clear_rect(0.0, 0.0, 600.0, 600.0);
            game.borrow().player.draw(&ctx);
            //console::log_1(&frame_name.into());
            }
        }) as Box<dyn FnMut()>);

        let _ = window
            .set_interval_with_callback_and_timeout_and_arguments_0(interval_callback
                                                                        .as_ref()
                                                                        .unchecked_ref(), 
                                                                        500);

        interval_callback.forget();
    }); //^-- spawn_local

    Ok(())
} //^--fn main_js

fn prepare_input(canvas: &web_sys::HtmlCanvasElement, game: Game) {
    let mut game = game.clone();
    let gamekeys: HashMap<String, i32> = HashMap::from([ ("ArrowUp".to_string(), 0), ("ArrowRight".to_string(),1) ,("ArrowDown".to_string(), 2),("ArrowLeft".to_string(), 3),(" ".to_string(), 4)]);  
 
    let onkeydown = Closure::wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        console::log_1(&keycode.key().into());
        game.set_keyevent(
            *gamekeys.get(&keycode.key()).unwrap_or(&5)); //5 is do nothing
        //game.get_keyevent().unwrap()        
        console::log_1(&format!("key {}", game.get_keyevent().unwrap()).into());
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    let onkeyup =
        Closure::wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {}) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    canvas.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));

    canvas.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));

    onkeydown.forget();
    onkeyup.forget();
}

fn set_common_style(ctx: &web_sys::CanvasRenderingContext2d) {
    ctx.set_shadow_color("#d53");
    ctx.set_shadow_blur(20.0);
    ctx.set_line_join("bevel");
    ctx.set_line_width(5.0);
}
