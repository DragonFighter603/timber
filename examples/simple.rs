#![feature(macro_metavar_expr)]

#[macro_use]
extern crate timber;

#[cfg(not(debug_assertions))]
macro_rules! println {
    ($($arg: tt)*) => {
        ()
    };
}

fn main() {
    log!("global {}", 0);
    log!(entry: "hello, world!");
    log!(main: "Running!");
    log!(main/computor: "Setting up computation...");
    debug!(main/computor: "Computing... {}%", 0);
    debug!(main/computor: "Computing... {}%", 42);
    debug!(main/computor: "Computing... {}%", 69);
    debug!(main/computor: "Computing... {}%", 69.0001);
    error!(main/computor: "Failed to compute :(");
    warn!(main: concat!("Finshed computating with ", colored!(BOLD RED_FG => "error"), ", using default values"));
    trace!(main: "internal error id is wadwdökjalkwdjlawjd")
}
