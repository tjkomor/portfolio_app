use std::env;
mod cgservice;
 
fn main() {
    let args: Vec<String> =  env::args().collect();
    let token: &str = &args[1];
    cgservice::get_token(token).ok();
}
