use std::collections::HashMap;

use aerugo::wm::types::{
    Image, KeyFilter, KeyModifiers, KeyStatus, Output, OutputId, Server, Toplevel, ToplevelConfigure, ToplevelId,
    ToplevelUpdates,
};
use exports::aerugo::wm::wm::{Guest, GuestWm};
use wit_bindgen::{rt::string::String, Resource};
use xkeysym::KeyCode;

pub struct Wm {
    /// All known toplevels.
    toplevels: HashMap<ToplevelId, Toplevel>,
}

impl Wm {
    fn new() -> Self {
        todo!()
    }

    fn new_toplevel(&mut self, _server: &Server, toplevel: Toplevel) {
        let id = toplevel.id();
        let toplevel = self.toplevels.entry(id).or_insert(toplevel);

        let _configure = ToplevelConfigure::new(toplevel);
    }

    fn closed_toplevel(&mut self, _server: &Server, toplevel: ToplevelId) {
        // The wm may keep the toplevel around for animations. For the example drop the toplevel handle.
        self.toplevels.remove(&toplevel);
    }

    fn update_toplevel(&mut self, _server: &Server, _toplevel: ToplevelId, _updates: ToplevelUpdates) {
        todo!()
    }

    fn ack_toplevel(&mut self, _server: &Server, _toplevel: ToplevelId, _serial: u32) {
        todo!()
    }

    fn committed_toplevel(&mut self, _server: &Server, _toplevel: ToplevelId, _image: Option<Image>) {
        todo!()
    }

    fn key(
        &mut self,
        _server: &Server,
        _time: u32,
        _key_code: KeyCode,
        _compose: Option<String>,
        _status: KeyStatus,
    ) -> KeyFilter {
        todo!()
    }

    fn key_modifiers(&mut self, _server: &Server, _modifiers: KeyModifiers) {
        todo!()
    }

    fn new_output(&mut self, _server: &Server, _output: Output) {
        todo!()
    }

    fn disconnect_output(&mut self, _server: &Server, _output: OutputId) {
        todo!()
    }
}

wit_bindgen::generate!({
    path: "../../wm.wit",

    world: "aerugo-wm",

    exports: {
        "aerugo:wm/wm": WmImpl,
        "aerugo:wm/wm/wm": WmImpl,
    },
});

pub struct WmImpl(std::cell::RefCell<Wm>);

impl Guest for WmImpl {
    fn create_wm(_server: &Server) -> Result<Resource<WmImpl>, String> {
        let wm = Wm::new();
        Ok(Resource::new(Self(std::cell::RefCell::new(wm))))
    }
}

impl GuestWm for WmImpl {
    fn new_toplevel(&self, server: &Server, toplevel: Toplevel) {
        self.0.borrow_mut().new_toplevel(server, toplevel);
    }

    fn closed_toplevel(&self, server: &Server, toplevel: ToplevelId) {
        self.0.borrow_mut().closed_toplevel(server, toplevel);
    }

    fn update_toplevel(&self, server: &Server, toplevel: ToplevelId, updates: ToplevelUpdates) {
        self.0.borrow_mut().update_toplevel(server, toplevel, updates);
    }

    fn ack_toplevel(&self, server: &Server, toplevel: ToplevelId, serial: u32) {
        self.0.borrow_mut().ack_toplevel(server, toplevel, serial);
    }

    fn committed_toplevel(&self, server: &Server, toplevel: ToplevelId, image: Option<Image>) {
        self.0.borrow_mut().committed_toplevel(server, toplevel, image)
    }

    fn key(&self, server: &Server, time: u32, sym: u32, compose: Option<String>, status: KeyStatus) -> KeyFilter {
        self.0
            .borrow_mut()
            .key(server, time, KeyCode::from(sym), compose, status)
    }

    fn key_modifiers(&self, server: &Server, modifiers: KeyModifiers) {
        self.0.borrow_mut().key_modifiers(server, modifiers)
    }

    fn new_output(&self, server: &Server, output: Output) {
        self.0.borrow_mut().new_output(server, output);
    }

    fn disconnect_output(&self, server: &Server, output: OutputId) {
        self.0.borrow_mut().disconnect_output(server, output);
    }
}
