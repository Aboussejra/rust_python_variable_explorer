use rust_python_variable_explorer::python_interpreter::get_variable_info_from_python_code;
#[macro_use] extern crate rocket;

#[get("/get_python_code", data = "<python_code>")]
fn hello(python_code: &str) -> String {
    get_variable_info_from_python_code(python_code);
    format!("Python code received is {}", python_code)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
