extern crate js;

use js::jsapi::CallArgs;

fn send(args: &CallArgs) {
    args.get(1);
}

fn main() {
    println!("hello world!");
}
