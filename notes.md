March 01

## 18h15

We have a basic model. A Player entity that has a position (x,y) and width and height.

### First let's create a player and show its position

- setup html, js. 
- Starting function from rs
- create player and display pos in console

## 19h30

Player is updated

March 02

## 16h55

HtmlCanvasElement: Clone  
so you can just clone it before moving it into the closure. 

For variables you want mutably shared between an event listener  
and other code you will typically wrap in a Rc<Cell> or Rc<RefCell>.

---

Basically Closure takes a rust closure that doesn't have any references to it's environment (which is why you need move) and stores it somewhere semi-permanently. Wasm_bindgen takes care of mapping that stored rust closure to the JavaScript callback that gets passed to the matching JavaScript API. That JavaScript API call is what actually sets up the callback with the DOM canvas object.

So the closure is called by JavaScript by way of invoking a JS callback, which calls into wasm_bindgen, which translates the callback to the appropriate rust closure.

forget transfers ownership of the closure to the JavaScript garbage collector, which is important here since you're passing the closure into JS and not keeping it around in any capacity. In practice this apparently leaks memory at least until weak references are accepted to the JS standard according to the docs.

---

Calling forget leaks memory, but the closure being called doesn't. The start function gets called once when the wasm module is instantiated so it isn't really leaking memory in practice. The JS callback would be keeping it alive anyway if you were working with pure JS.

In normal JavaScript the callback you set as an event handler will live as long as these conditions hold:

- The element you set it as an event handler on still exists
- The event handler hasn't been removed

This isn't plain JS though so things get a lot more complicated.

The code inside the closures will run every time the associated event fires in the DOM. The start function itself only runs once, but it sets up those closures to run in response to some events.

```rust
// start is executed automatically once when the wasm 
// module is loaded by the JavaScript engine
// https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/start.html
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Perform one time setup for the program
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    
    // Create the data structures that will be shared by the callbacks
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    {
        // Clone the Rcs so that we can move them into the closure
        let context = context.clone();
        let pressed = pressed.clone();

        // Create a closure that references our shared state.
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            // The contents of the closure are only run when the 
            // closure is called by the JS event handler. 
            // The code inside the closures is the only part of this 
            // program that runs repeatedly.
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        });

        // Register our closure as an event handler on the 
        // canvas DOM element 
        // (indirectly via JavaScript, this is managed by wasm_bindgen)
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;

        // We need the closure to be retained since we passed it to JS, 
        // and JS doesn't know how to retain rust data.
        closure.forget();
    }
    // Additional code omitted...
}
```

---


