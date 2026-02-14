use std::future::Future;

use godot::{classes::object::ConnectFlags, prelude::*};


#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct Communicator {
    is_done : bool,

    base : Base<RefCounted>,
}


#[godot_api]
impl Communicator {
    #[signal]
    pub fn done();


    #[func]
    pub fn new_gd() -> Gd<Self> {
        let communicator = Gd::from_init_fn(|base| {
            Self {
                is_done : false,
                base
            }
        });

        communicator
            .signals()
            .done()
            .builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect_self_mut(
                |me| {
                    me.is_done = true;
                }
            );
        
        communicator
    }
    

    pub fn get_done_future(&self) -> Option<impl Future<Output = ()>> {
        if self.is_done {
            return None;
        }

        // Else
        let me = self.to_gd();
        let future = async move {
            let _await = me.signals().done().to_fallible_future().await;
        };

        Some(future)
    }
}
