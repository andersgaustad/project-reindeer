use crate::core::audio::sfx_entry::SFXEntry;


pub trait ISFXManager {
    fn play(&mut self, entry : SFXEntry);
}
