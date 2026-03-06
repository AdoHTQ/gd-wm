use wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New};
use wayland_protocols::xdg::shell::server::xdg_wm_base::{self, XdgWmBase};

use crate::state::State;


impl GlobalDispatch<XdgWmBase, ()> for State {
    fn bind(
        _state: &mut State,
        _handle: &DisplayHandle,
        _client: &Client,
        resource: New<XdgWmBase>,
        _global_data: &(),
        data_init: &mut DataInit<'_, State>,
    ) {
        data_init.init(resource, ());
    }
}

impl Dispatch<XdgWmBase, ()> for State {
    fn request(
        _state: &mut State,
        _client: &Client,
        _resource: &XdgWmBase,
        _request: xdg_wm_base::Request,
        _data: &(),
        _handle: &DisplayHandle,
        _data_init: &mut DataInit<'_, State>,
    ) {

    }
}