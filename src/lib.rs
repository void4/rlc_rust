use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "env")]
extern {
    fn color() -> i32;
    fn board(index: i32) -> i32;
    fn rights() -> i32;
    fn maxlegal() -> i32;
    fn randint(max: i32) -> i32;
    fn legal(index: i32) -> i32;
    fn log(msg: i32) -> i32;
}

#[no_mangle]
pub extern fn r#move() -> i32 {
    for i in 0..maxlegal() {
        let piece = board(legal(i) & (64-1));
        if piece == 1 || piece == 2 {
            return legal(i);
        }
    }
    return legal(randint(maxlegal()));
}
