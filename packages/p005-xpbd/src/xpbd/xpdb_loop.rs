use bevy::{ecs::schedule::ShouldRun, prelude::*};

use super::consts::{DELTA_TIME, NUM_SUBSTEPS};

// TODO: use https://github.com/IyesGames/iyes_loopless instead
#[derive(Debug, Default, Resource)]
pub struct XpbdLoop {
    pub(crate) has_added_time: bool,
    pub(crate) accumulator: f32,
    pub(crate) substepping: bool,
    pub(crate) current_substep: u32,
    pub(crate) queued_steps: u32,
    pub paused: bool,
}

impl XpbdLoop {
    pub fn _step(&mut self) {
        self.queued_steps += 1;
    }
    pub fn _pause(&mut self) {
        self.paused = true;
    }
    pub fn _resume(&mut self) {
        self.paused = false;
    }
}

pub fn _pause(mut xpbd_loop: ResMut<XpbdLoop>) {
    xpbd_loop._pause();
}

pub fn _resume(mut xpbd_loop: ResMut<XpbdLoop>) {
    xpbd_loop._resume();
}

pub fn run_criteria(time: Res<Time>, mut state: ResMut<XpbdLoop>) -> ShouldRun {
    if state.paused && state.queued_steps == 0 {
        return ShouldRun::No;
    }

    if !state.has_added_time {
        state.has_added_time = true;

        if state.paused {
            state.accumulator += DELTA_TIME * state.queued_steps as f32;
        } else {
            state.accumulator += time.delta_seconds();
        }
    }

    if state.substepping {
        state.current_substep += 1;

        if state.current_substep < NUM_SUBSTEPS {
            return ShouldRun::YesAndCheckAgain;
        } else {
            // We finished a whole step
            if state.paused && state.queued_steps > 0 {
                state.queued_steps -= 1;
            } else {
                state.accumulator -= DELTA_TIME;
            }

            state.current_substep = 0;
            state.substepping = false;
        }
    }

    if state.accumulator >= DELTA_TIME {
        state.substepping = true;
        state.current_substep = 0;
        ShouldRun::YesAndCheckAgain
    } else {
        state.has_added_time = false;
        ShouldRun::No
    }
}

pub fn first_substep(state: Res<XpbdLoop>) -> ShouldRun {
    if state.current_substep == 0 {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn last_substep(state: Res<XpbdLoop>) -> ShouldRun {
    if state.current_substep == NUM_SUBSTEPS - 1 {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
