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


    pub async fn await_done(&self) {
        if self.is_done {
            return;
        }

        let me = self.to_gd();

        // AWAIT
        let _await = me.signals().done().to_fallible_future().await;
        // AWAIT
    }
}
