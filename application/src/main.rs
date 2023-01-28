use app::{ApplicationState, TheApplication};
use shadow_rs::shadow;
use std::{collections::HashMap, io};
use utils::config::CONFIG;

pub mod app;
pub mod error;
pub mod plugin;
pub mod utils;

shadow!(build);

fn main() -> io::Result<()> {
    let mut plugins = HashMap::new();
    let mut state = ApplicationState::new();

    let loaded_libraries = plugin::check(&CONFIG.plugins, &mut state);
    plugin::load(&mut plugins, &mut state, loaded_libraries);

    let mut app = TheApplication { plugins, state };

    let mut config_commands = CONFIG.commands.vec.clone().into_iter();
    while !app.is_finished() {
        if let Some((plugin_id, command)) = config_commands.next() {
            let command = command.get();
            if let Err(e) = app.run_command(&plugin_id, command.into()) {
                eprintln!(
                    "Error while running command on:\n{:?}\nError:{}\nCommand:\n{:?}\n",
                    plugin_id, e, command
                );
            }
        }

        if let Err(e) = app.tick() {
            eprintln!("Error in application loop:\n{}\n", e);
        }
    }

    if app.is_finished() {
        println!("timeout waiting for events");
    }

    Ok(())
}
