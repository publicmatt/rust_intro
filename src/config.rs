use std::env;
pub struct CalcConfig {
    lines: i32,
    text: String,
}
impl CalcConfig {
    pub fn new(mut args: env::Args) -> Result<CalcConfig, &'static str> {
        args.next();
        let lines: i32 = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => return Err("number of lines not supplied"),
        };
        let text: String = match args.next() {
            Some(arg) => arg,
            None => return Err("text not supplied"),
        };
        Ok(CalcConfig { lines, text })
    }
}
