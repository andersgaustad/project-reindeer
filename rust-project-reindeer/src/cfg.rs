pub const WEB_BUILD_BUTTON_DISABLED_TOOLTIP : &str = "Exit button disabled in Web builds; close the browser to exit.";


#[inline(always)]
pub const fn is_web_build() -> bool {
    cfg!(target_arch = "wasm32")
}


#[inline(always)]
pub const fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
