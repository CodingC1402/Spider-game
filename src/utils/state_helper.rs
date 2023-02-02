use bevy::{ecs::schedule::StateData, prelude::*};
use iyes_loopless::{condition::ConditionalSystemDescriptor, prelude::*};

pub trait StateExtend {
    fn add_system_run_if<Params, State: StateData>(
        &mut self,
        state: Option<State>,
        system: impl IntoSystem<(), (), Params>,
    ) -> &mut Self;

    fn add_startup_system_if_state<Params, State: StateData>(
        &mut self,
        state: Option<State>,
        system: impl IntoSystem<(), (), Params>,
    ) -> &mut Self;
}
impl StateExtend for App {
    fn add_system_run_if<Params, State: StateData>(
        &mut self,
        state: Option<State>,
        system: impl IntoSystem<(), (), Params>,
    ) -> &mut Self {
        match state {
            Some(x) => self.add_system(system.run_in_state(x)),
            None => self.add_system(system),
        }
    }

    fn add_startup_system_if_state<Params, State: StateData>(
        &mut self,
        state: Option<State>,
        system: impl IntoSystem<(), (), Params>,
    ) -> &mut Self {
        match state {
            Some(x) => self.add_enter_system(x, system),
            None => self.add_startup_system(system),
        }
    }
}

pub trait RunIfExtend<Params>: IntoSystem<(), (), Params> + Sized {
    fn run_in_some_state<State: StateData>(
        self,
        state: Option<State>,
    ) -> ConditionalSystemDescriptor {
        match state {
            Some(x) => self.run_in_state(x),
            None => self.into_conditional(),
        }
    }
}
impl<S, Params> RunIfExtend<Params> for S where S: IntoSystem<(), (), Params> {}
