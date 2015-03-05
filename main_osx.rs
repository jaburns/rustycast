#![no_main]

mod main;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn SDL_main() {
    main::real_main();
}
