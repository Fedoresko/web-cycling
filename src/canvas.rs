use std::cell::RefCell;
use std::rc::Rc;

use serde_json;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;

use crate::messaging::Msg;
use crate::WebEventDispatcher;
use crate::app::ui::messaging::EventTarget;

pub static APP_DIV_ID: &'static str = "web-cycling";

pub(crate) type EventDispatcher<'a> = &'a Rc<RefCell<Option<WebEventDispatcher>>>;

pub fn send_msg(dispatcher: EventDispatcher, msg: &Msg) {
    dispatcher.as_ref()
        .borrow_mut()
        .as_mut()
        .unwrap()
        .msg(msg);
}

pub fn create_webgl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    let result = JsValue::from_serde(&serde_json::json!({
        "antialias": false,
    }));
    let gl: WebGl2RenderingContext = canvas
        .get_context_with_context_options("webgl2", &result.unwrap())?
        .unwrap()
        .dyn_into()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.enable(GL::DEPTH_TEST);

    Ok(gl)
}

pub fn init_canvas(event_dispatcher: EventDispatcher) -> Result<HtmlCanvasElement, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(window.inner_width()?.as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height()?.as_f64().unwrap() as u32);

    attach_mouse_down_handler(&canvas, event_dispatcher)?;
    attach_mouse_up_handler(&canvas, event_dispatcher)?;
    attach_mouse_move_handler(&canvas, event_dispatcher)?;
    attach_mouse_wheel_handler(&canvas, event_dispatcher)?;

    attach_touch_start_handler(&canvas, event_dispatcher)?;
    attach_touch_move_handler(&canvas, event_dispatcher)?;
    attach_touch_end_handler(&canvas, event_dispatcher)?;
    attach_key_handler(event_dispatcher)?;
    attach_key_up_handler(event_dispatcher)?;
    attach_size_handler(&canvas, event_dispatcher)?;

    let app_div: HtmlElement = match document.get_element_by_id(APP_DIV_ID) {
        Some(container) => container.dyn_into()?,
        None => {
            let app_div = document.create_element("div")?;
            app_div.set_id(APP_DIV_ID);
            app_div.dyn_into()?
        }
    };

    app_div.style().set_property("display", "flex")?;
    app_div.append_child(&canvas)?;

    Ok(canvas)
}

fn attach_mouse_down_handler(
    canvas: &HtmlCanvasElement,
    event_dispatcher: EventDispatcher,
) -> Result<(), JsValue> {
    let event_dispatcher = Rc::clone(event_dispatcher);
    let handler = move |event: MouseEvent| {
        let x = event.client_x();
        let y = event.client_y();
        send_msg(&event_dispatcher, &Msg::MouseDown(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}

fn attach_mouse_up_handler(
    canvas: &HtmlCanvasElement,
    event_dispatcher: EventDispatcher,
) -> Result<(), JsValue> {
    let event_dispatcher = Rc::clone(event_dispatcher);
    let handler = move |_event: MouseEvent| {
        let x = _event.client_x();
        let y = _event.client_y();
        send_msg(&event_dispatcher, &Msg::MouseUp(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    handler.forget();
    Ok(())
}

fn attach_mouse_move_handler(
    canvas: &HtmlCanvasElement,
    event_dispatcher: EventDispatcher,
) -> Result<(), JsValue> {
    let event_dispatcher = Rc::clone(event_dispatcher);
    let handler = move |event: MouseEvent| {
        event.prevent_default();
        let x = event.client_x();
        let y = event.client_y();
        send_msg(&event_dispatcher, &Msg::MouseMove(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_wheel_handler(
    canvas: &HtmlCanvasElement,
    event_dispatcher: EventDispatcher,
) -> Result<(), JsValue> {
    let event_dispatcher = Rc::clone(event_dispatcher);
    let handler = move |event: WheelEvent| {
        event.prevent_default();

        let zoom_amount = event.delta_y() / 50.;

        send_msg(&event_dispatcher, &Msg::Zoom(zoom_amount as f32));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("wheel", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_touch_start_handler(
    canvas: &HtmlCanvasElement,
    event_dispatcher: EventDispatcher,
) -> Result<(), JsValue> {
    let event_dispatcher = Rc::clone(event_dispatcher);
    let handler = move |event: TouchEvent| {
        let touch = event.touches().item(0).expect("First Touch");
        let x = touch.client_x();
        let y = touch.client_y();
        send_msg(&event_dispatcher, &Msg::MouseDown(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchstart", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_key_up_handler(event_dispatcher: EventDispatcher) -> Result<(), JsValue> {
    let event_dispatcher = Rc::clone(event_dispatcher);
    let handler = move |event: KeyboardEvent| {
        send_msg(&event_dispatcher, &Msg::KeyUp(event.key_code()));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    window()
        .unwrap()
        .document()
        .unwrap()
        .add_event_listener_with_callback("keyup", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_key_handler(event_dispatcher: EventDispatcher) -> Result<(), JsValue> {
    let event_dispatcher = Rc::clone(event_dispatcher);
    let handler = move |event: KeyboardEvent| {
        send_msg(&event_dispatcher, &Msg::KeyDown(event.key_code()));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    window()
        .unwrap()
        .document()
        .unwrap()
        .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_size_handler(
    _canvas: &HtmlCanvasElement,
    event_dispatcher: EventDispatcher,
) -> Result<(), JsValue> {
    let event_dispatcher = Rc::clone(event_dispatcher);
    let handler = move || {
        let window = window().unwrap();
        let w = window.inner_width().unwrap().as_f64().unwrap();
        let h = window.inner_height().unwrap().as_f64().unwrap();
        send_msg(&event_dispatcher, &Msg::ResizeViewport(w as i32, h as i32));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut()>);
    window()
        .unwrap()
        .set_onresize(Some(handler.as_ref().unchecked_ref()));
    handler.forget();

    Ok(())
}

fn attach_touch_move_handler(
    canvas: &HtmlCanvasElement,
    event_dispatcher: EventDispatcher,
) -> Result<(), JsValue> {
    let event_dispatcher = Rc::clone(event_dispatcher);
    let handler = move |event: TouchEvent| {
        event.prevent_default();
        let touch = event.touches().item(0).expect("First Touch");
        let x = touch.client_x();
        let y = touch.client_y();
        send_msg(&event_dispatcher, &Msg::MouseMove(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchmove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_touch_end_handler(
    canvas: &HtmlCanvasElement,
    event_dispatcher: EventDispatcher,
) -> Result<(), JsValue> {
    let event_dispatcher = Rc::clone(event_dispatcher);
    let handler = move |_event: TouchEvent| {
        let touch = _event.touches().item(0).expect("First Touch");
        let x = touch.client_x();
        let y = touch.client_y();
        send_msg(&event_dispatcher, &Msg::MouseUp(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("touchend", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}
