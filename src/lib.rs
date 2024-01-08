#![feature(macro_metavar_expr)]

#[macro_export]
macro_rules! __format_log {
    ($level: ident, $($origin: ident)/ * : $formatter: expr, $($arg: tt)*) => {
        format!(
            concat!("{} [", colored!(BOLD => "{}"), "] ", $formatter), 
            colored!(get_log_attr!($level->COLOR) => get_log_attr!($level->NAME_UPPER)), 
            stringify!($($origin)/+), $($arg)*
        )
    };
}

macro_rules! create_log_levels {
    ($($level: ident $level_upper: ident $color: expr);*;) => {
        $(
            #[macro_export]
            macro_rules! $level {
                ($$($$arg: tt)*) => {
                    println!("{}", $crate::__format_log!($level, $$($$arg)*,))
                };
            }
        )*
        #[macro_export]
        macro_rules! get_log_attr {
            $(
                ($level -> COLOR) => { $color };
                ($level -> NAME) => { stringify!($level) };
                ($level -> NAME_UPPER) => { stringify!($level_upper) };
            )*
        }
    };
}

create_log_levels!(
    error ERROR (255,   0,   0);
    warn  WARN  (255, 255,   0);
    log   LOG   (  0, 255, 255);
    debug DEBUG (255, 255, 255);
    trace TRACE (200, 200, 200);
);

#[macro_export]
macro_rules! __static_rule {
    (BOLD)         => { "\x1b[1m" };
    (FAINT)        => { "\x1b[2m" };
    (ITALIC)       => { "\x1b[3m" };
    (UNDERLINE)    => { "\x1b[4m" };
    (BLINKING)     => { "\x1b[5m" };
    (INVERSE)      => { "\x1b[7m" };
    (HIDDEN)       => { "\x1b[8m" };
    (STRIKETHROGH) => { "\x1b[9m" };

    (BLACK_FG)     => { "\x1b[30m" };
    (BLACK_BG)     => { "\x1b[40m" };
    (RED_FG)       => { "\x1b[31m" };
    (RED_BG)       => { "\x1b[41m" };
    (GREEN_FG)     => { "\x1b[32m" };
    (GREEN_BG)     => { "\x1b[42m" };
    (YELLOW_FG)    => { "\x1b[33m" };
    (YELLOW_BG)    => { "\x1b[43m" };
    (BLUE_FG)      => { "\x1b[34m" };
    (BLUE_BG)      => { "\x1b[44m" };
    (MAGENTA_FG)   => { "\x1b[35m" };
    (MAGENTA_BG)   => { "\x1b[45m" };
    (CYAN_FG)      => { "\x1b[36m" };
    (CYAN_BG)      => { "\x1b[46m" };
    (WHITE_FG)     => { "\x1b[37m" };
    (WHITE_BG)     => { "\x1b[47m" };
    (DEFAULT_FG)   => { "\x1b[38m" };
    (DEFAULT_BG)   => { "\x1b[48m" };
}

#[macro_export]
macro_rules! colored {
    ($($rule: ident)* => $message: literal) => {
        concat!($($crate::__static_rule!($rule)),* , $message, "\x1b[0m")
    };
    ($($rule: ident)* => $message: expr) => {
        format!("{}{}\x1b[0m", concat!($($crate::__static_rule!($rule)),*) , $message)
    };
    ($fg: expr => $message: expr) => {{
        let (r, g, b) = $fg;
        format!("\x1b[0m\x1b[38;2;{r};{g};{b}m{}\x1b[0m", $message)
    }};
    ($fg: expr, $bg: expr => $message: expr) => {{
        let (fr, fg, fb) = $fg;
        let (br, bg, bb) = $bg;
        format!("\x1b[0m\x1b[38;2;{fr};{fg};{fb}mx1b[48;2;{br};{bg};{bb}m{}\x1b[0m", $message)
    }}
}