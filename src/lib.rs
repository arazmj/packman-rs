mod utils;
mod game;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use game::Game;

#[wasm_bindgen]
extern "C" {
    fn game_over(score: u32);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("pacman-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let game = Rc::new(RefCell::new(Game::new()));
    
    // Input handling
    // Input handling (Keyboard)
    {
        let game = game.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            game.borrow_mut().set_direction(event.key_code());
        }) as Box<dyn FnMut(_)>);
        window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Input handling (Touch)
    {
        let game = game.clone();
        let start_x = Rc::new(RefCell::new(0.0));
        let start_y = Rc::new(RefCell::new(0.0));

        // Touch Start
        let start_x_clone = start_x.clone();
        let start_y_clone = start_y.clone();
        let touch_start = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            event.prevent_default(); // Prevent scrolling
            let touches = event.touches();
            if let Some(touch) = touches.get(0) {
                *start_x_clone.borrow_mut() = touch.client_x() as f64;
                *start_y_clone.borrow_mut() = touch.client_y() as f64;
            }
        }) as Box<dyn FnMut(_)>);
        window.add_event_listener_with_callback("touchstart", touch_start.as_ref().unchecked_ref())?;
        touch_start.forget();

        // Touch Move (Prevent scrolling)
        let touch_move = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            event.prevent_default(); 
        }) as Box<dyn FnMut(_)>);
        window.add_event_listener_with_callback("touchmove", touch_move.as_ref().unchecked_ref())?;
        touch_move.forget();

        // Touch End
        let touch_end = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            event.prevent_default(); // Prevent scrolling
            let touches = event.changed_touches();
            if let Some(touch) = touches.get(0) {
                let end_x = touch.client_x() as f64;
                let end_y = touch.client_y() as f64;
                let diff_x = end_x - *start_x.borrow();
                let diff_y = end_y - *start_y.borrow();

                if diff_x.abs() > diff_y.abs() {
                    // Horizontal swipe
                    if diff_x > 0.0 {
                        game.borrow_mut().set_direction(39); // Right
                    } else {
                        game.borrow_mut().set_direction(37); // Left
                    }
                } else {
                    // Vertical swipe
                    if diff_y > 0.0 {
                        game.borrow_mut().set_direction(40); // Down
                    } else {
                        game.borrow_mut().set_direction(38); // Up
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        window.add_event_listener_with_callback("touchend", touch_end.as_ref().unchecked_ref())?;
        touch_end.forget();
    }

    // Game Loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let game_loop = game.clone();
    let mut game_over_triggered = false;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game_loop.borrow_mut().tick();
        game_loop.borrow().draw(&context);

        if game_loop.borrow().game_over && !game_over_triggered {
            game_over_triggered = true;
            game_over(game_loop.borrow().get_score());
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
