use core_extensions::StringExt;
use std::{
    collections::{HashMap, VecDeque},
    mem,
    time::{Duration, Instant},
};

use abi_stable::{
    external_types::crossbeam_channel::{self, RReceiver, RSender},
    sabi_trait::TD_Opaque,
    std_types::{RArc, ROption::RSome, RStr},
};
use common::{
    Application, Application_TO, Error as AppError, PluginCommand, PluginFactory_Ref, PluginId,
    PluginResponse, PluginType,
};

pub struct TheApplication {
    pub(super) plugins: HashMap<PluginId, PluginType>,
    pub(super) state: ApplicationState,
}

pub struct ApplicationState {
    pub(super) id_map: HashMap<PluginId, PluginFactory_Ref>,
    pub(super) commands: VecDeque<RArc<PluginCommand>>,
    pub(super) responses: VecDeque<RArc<PluginResponse>>,
    pub(super) sender: RSender<PluginCommand>,
    pub(super) receiver: RReceiver<PluginCommand>,
    pub(super) last_run_at: Instant,
}

fn print_response(plugin_id: &PluginId, response: &str) {
    println!(
        "reponse:\n{}\nfrom:\n    {:?}\n\n",
        response.left_pad(4),
        plugin_id,
    );
}

impl TheApplication {
    pub fn run_command(&mut self, plugin_id: &PluginId, command: RStr<'_>) -> Result<(), AppError> {
        let state = Application_TO::from_ptr(&mut self.state, TD_Opaque);
        let plugin = self.plugins.get_mut(plugin_id).unwrap();
        let resp = plugin.send_command(command, state).into_result()?;
        self.state.register_command_run();
        print_response(&plugin_id, &resp);
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), AppError> {
        if let Ok(command) = self.state.receiver.try_recv() {
            self.state.send_command_to_plugin(RArc::new(command));
        }

        if let Some(command) = self.state.commands.pop_front() {
            self.run_command_(command)?;
        }

        let mut responses = mem::replace(&mut self.state.responses, VecDeque::new());
        for response in responses.drain(..) {
            let state = Application_TO::from_ptr(&mut self.state, TD_Opaque);
            let plugin_id = response.to.clone();
            let plugin = self.plugins.get_mut(&plugin_id).unwrap();
            if let RSome(res) = plugin
                .handle_response(response.clone(), state)
                .into_result()?
            {
                print_response(&plugin_id, &res.response);
            }
        }
        self.state.responses = responses;

        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        self.state.last_run_at.elapsed() >= Duration::from_secs(5)
    }

    fn run_command_(&mut self, plugin_command: RArc<PluginCommand>) -> Result<(), AppError> {
        let state = Application_TO::from_ptr(&mut self.state, TD_Opaque);
        let plugin_id = plugin_command.to.clone();
        let plugin = self.plugins.get_mut(&plugin_id).unwrap();
        let response = plugin
            .send_command(plugin_command.command.as_rstr(), state)
            .into_result()?;

        self.state.register_command_run();

        let response = PluginResponse {
            from: plugin_command.from.clone(),
            to: plugin_command.to.clone(),
            response,
        };

        self.state.responses.push_back(RArc::new(response));
        Ok(())
    }
}

impl ApplicationState {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();

        Self {
            id_map: HashMap::new(),
            commands: VecDeque::new(),
            responses: VecDeque::new(),
            sender,
            receiver,
            last_run_at: Instant::now(),
        }
    }

    fn register_command_run(&mut self) {
        self.last_run_at = Instant::now();
    }
}

impl Application for ApplicationState {
    fn send_command_to_plugin(&mut self, command: RArc<PluginCommand>) {
        self.commands.push_back(command);
    }

    fn sender(&self) -> RSender<PluginCommand> {
        self.sender.clone()
    }
}
