#[macro_use]
extern crate serde_derive;

pub mod error;
pub mod openc2;
pub mod util;

use abi_stable::{
    declare_root_module_statics,
    external_types::crossbeam_channel::RSender,
    library::RootModule,
    package_version_strings, sabi_trait,
    sabi_types::{RMut, VersionStrings},
    std_types::{RArc, RBox, ROk, ROption, RResult, RSome, RStr, RString},
    StableAbi,
};
pub use error::Error;

pub type PluginId = RString;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, StableAbi)]
pub struct PluginCommand {
    pub from: PluginId,
    pub to: PluginId,
    pub command: RString,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, StableAbi)]
pub struct PluginResponse {
    pub from: PluginId,
    pub to: PluginId,
    pub response: RString,
}

pub type PluginType = Plugin_TO<'static, RBox<()>>;

/// A plugin which is loaded by the application,and provides some functionality.
#[sabi_trait]
//#[sabi(debug_print)]
pub trait Plugin {
    /// Handles a JSON encoded command.
    fn send_command(
        &mut self,
        command: RStr<'_>,
        app: ApplicationMut<'_>,
    ) -> RResult<RString, Error>;

    /// Handles a response from another Plugin,
    /// from having called `ApplicationMut::send_command_to_plugin` ealier.
    fn handle_response(
        &mut self,
        response: RArc<PluginResponse>,
        _app: ApplicationMut<'_>,
    ) -> RResult<ROption<RArc<PluginResponse>>, Error> {
        ROk(RSome(response))
    }

    /// Gets the PluginId that was passed to this plugin in its constructor.
    fn plugin_id(&self) -> &PluginId;

    /// Closes the plugin,
    ///
    /// This does not unload the dynamic library of this plugin,
    /// you can instantiate another instance of this plugin with
    /// `PluginFactory_Ref::get_module().new()(application_handle)`.
    ///
    ///
    ///
    /// The `#[sabi(last_prefix_field)]` attribute here means that this is the last method
    /// that was defined in the first compatible version of the library
    /// (0.1.0, 0.2.0, 0.3.0, 1.0.0, 2.0.0 ,etc),
    /// requiring new methods to always be added below preexisting ones.
    ///
    /// The `#[sabi(last_prefix_field)]` attribute would stay on this method until the library
    /// bumps its "major" version,
    /// at which point it would be moved to the last method at the time.
    #[sabi(last_prefix_field)]
    fn close(self, app: ApplicationMut<'_>);
}

///////////////////////////////////////////////////////////////////////////////

/// The root module of a`plugin` dynamic library.
///
/// To load this module,
/// call <PluginFactory as RootModule>::load_from_directory(some_directory_path)
#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = "PluginFactory_Ref")))]
#[sabi(missing_field(panic))]
pub struct PluginFactory {
    /// Constructs the plugin.
    ///
    ///
    /// The `#[sabi(last_prefix_field)]` attribute here means that this is the last field in this struct
    /// that was defined in the first compatible version of the library
    /// (0.1.0, 0.2.0, 0.3.0, 1.0.0, 2.0.0 ,etc),
    /// requiring new fields to always be added below preexisting ones.
    ///
    /// The `#[sabi(last_prefix_field)]` attribute would stay on this field until the library
    /// bumps its "major" version,
    /// at which point it would be moved to the last field at the time.
    ///
    #[sabi(last_prefix_field)]
    pub new: extern "C" fn(RSender<PluginCommand>, PluginId) -> RResult<PluginType, Error>,
}

impl RootModule for PluginFactory_Ref {
    declare_root_module_statics! {PluginFactory_Ref}
    const BASE_NAME: &'static str = "plugin";
    const NAME: &'static str = "plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

pub type ApplicationMut<'a> = Application_TO<'a, RMut<'a, ()>>;

#[sabi_trait]
pub trait Application {
    fn send_command_to_plugin(&mut self, command: RArc<PluginCommand>);

    fn sender(&self) -> RSender<PluginCommand>;
}
