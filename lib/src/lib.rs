use wasm_bindgen::prelude::*;
mod complex;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn ping(name: &str) -> String {
    log(&format!("Hello, {}!", name));
    //alert(&format!("Hello, {}!", name));


    return format!("Hello, {}!", name);
}

#[wasm_bindgen]
pub fn test(name: &str) -> String {
    log(&format!("Hello, {}!", name));
    //alert(&format!("Hello, {}!", name));

    let mut c1 = complex::complex::ComplexType {
        imag: 1.0,
        real: 3.0
    };

    let mut c2 = complex::complex::ComplexType {
        imag: 6.0,
        real: 6.0
    };

    let result = complex::complex::add(&c1, &c2);
    let result1 = complex::complex::mul(&c1, &c2);
    let result2 = complex::complex::sub(&c1, &c2);
    let result3 = complex::complex::cexp(&c1);

    log(&result.imag.to_string());
    log(&result.real.to_string());

    log(&result1.imag.to_string());
    log(&result1.real.to_string());

    log(&result2.imag.to_string());
    log(&result2.real.to_string());

    log(&result3.imag.to_string());
    log(&result3.real.to_string());

    return format!("Hello, {}!", name);
}