#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        let input = include_bytes!("../reona.webp");
        let mut width: std::os::raw::c_int = 0;
        let mut height: std::os::raw::c_int = 0;
        let res = WebPGetInfo(input as _, input.len() as _, &mut width, &mut height);

        match res {
            0 => panic!("Failed to get info"),
            1 => {
                println!("Image width: {}, height: {}", width, height);
            }
            other => panic!("Unknown return value: {}", other),
        }
    }
}
