#[macro_use]
extern crate neon;
extern crate num_cpus;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn thread_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

// arrays
fn convert_vev_to_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    let vec: Vec<String> = Vec::with_capacity(100);

    let js_array = JsArray::new(&mut cx, vec.len() as u32);

    for (i, obj) in vec.iter().enumerate() {
        let js_string = cx.string(obj);
        js_array.set(&mut cx, i as u32, js_string).unwrap();
    }
    Ok(js_array)
}

fn convert_js_array_to_vec(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;

    let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
    Ok(cx.number(vec.len() as f64))
}

fn return_js_array_with_number(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array: Handle<JsArray> = JsArray::new(&mut cx, 1);
    let n = cx.number(9000.0);
    array.set(&mut cx, 0, n)?;
    Ok(array)
}

fn return_js_array_with_string(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array: Handle<JsArray> = JsArray::new(&mut cx, 1);
    let s = cx.string("Hello nodejs");
    array.set(&mut cx, 0, s)?;
    Ok(array)
}


// objects


// simple args

register_module!(mut cx, {
    cx.export_function("hello", hello);
    cx.export_function("threadCount" , thread_count)
});
