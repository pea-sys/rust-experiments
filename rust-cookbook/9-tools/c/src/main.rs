use std::ffi::CString;
use std::os::raw::c_char;
use std::error;
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
extern {
    fn hello();
    fn greet(name: *const c_char);
}

fn run() -> Result<()> {
    unsafe { hello() }
    let name = "pea-sys";
    let c_name = CString::new(name)?;
    unsafe { greet(c_name.as_ptr()) }
    Ok(())
}
fn main(){
    run();
}