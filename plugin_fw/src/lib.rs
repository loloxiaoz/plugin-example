use abi_stable::{
    export_root_module,
    external_types::crossbeam_channel::RSender,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    sabi_trait::prelude::TD_Opaque,
    std_types::{ROk, RResult, RStr, RString},
};

use common::{
    ApplicationMut, Error as AppError, Plugin, PluginCommand, PluginFactory, PluginFactory_Ref,
    PluginId, PluginType, Plugin_TO,
};

///////////////////////////////////////////////////////////////////////////////////

/// Exports the root module of this library.
///
/// This code isn't run until the layout of the type it returns is checked.
#[export_root_module]
fn instantiate_root_module() -> PluginFactory_Ref {
    PluginFactory { new }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn new(_sender: RSender<PluginCommand>, plugin_id: PluginId) -> RResult<PluginType, AppError> {
    let this = PluginFireWall { plugin_id };
    ROk(Plugin_TO::from_value(this, TD_Opaque))
}

struct PluginFireWall {
    plugin_id: PluginId,
}

impl Plugin for PluginFireWall {
    fn send_command(
        &mut self,
        command: RStr<'_>,
        _app: ApplicationMut<'_>,
    ) -> RResult<RString, AppError> {
        println!("command:\n{}", command);
        ROk(RString::from("send messge to plugin firewall success"))
    }

    fn plugin_id(&self) -> &PluginId {
        &self.plugin_id
    }

    fn close(self, _app: ApplicationMut<'_>) {}
}
