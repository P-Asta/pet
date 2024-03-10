use std::time::Duration;

use crate::state::State;
use bevy::{
    prelude::*,
    render::render_resource::encase::rts_array::Length,
    transform,
    window::{PrimaryWindow, WindowResized},
    winit::WinitWindows,
};
use rand::Rng;
#[derive(Component, Clone)]
pub struct Pet {
    state: Vec<State>,
    pub pos: Vec2,
    pub size: Vec2,
    timer: Timer,
    type_: u8,
}
pub static mut PetPos: Vec2 = Vec2::new(300., 10000.);
static mut PetSpeed: f32 = 3.;

impl Pet {
    pub fn new(type_: u8) -> Self {
        Self {
            state: vec![State::Jump],
            pos: Vec2::new(0., 0.),
            size: Vec2::new(100., 100.),
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            type_,
        }
    }
}

pub struct PetPlugin;

impl Plugin for PetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_info)
            .add_systems(Update, gravity)
            .add_systems(Update, manage_state)
            .add_systems(Update, boxin)
            // .add_systems(Update, click)
            .add_systems(Update, movement);
    }
}
pub fn change_info(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut q: Query<(&mut Pet, Entity, &mut Style), With<Pet>>,
) {
    let mut state = State::Jump;
    for (mut pet, entity, mut style) in &mut q {
        if pet.type_ == 1 {
            if pet.state.length() > 0 {
                state = pet.state[0]
            }
        } else {
            match state {
                State::Left => pet.pos = Vec2::new(-10., 0.),
                State::Right => pet.pos = Vec2::new(10., 0.),
                _ => pet.pos = Vec2::new(0., 0.),
            }
        }

        let mut component = commands.entity(entity);
        style.width = Val::Px(pet.size.x);
        style.height = Val::Px(pet.size.y);

        if let (Val::Px(top), Val::Px(left)) = (style.top, style.left) {
            style.top = Val::Px(top + (unsafe { PetPos.y } + pet.pos.y - top) / 10.);
            style.left = Val::Px(left + (unsafe { PetPos.x } + pet.pos.x - left) / 10.);
        } else {
            style.top = Val::Px(unsafe { PetPos.y });
            style.left = Val::Px(unsafe { PetPos.x });
        }
    }
}

fn movement(
    windows: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut q: Query<(&mut Pet, &mut Transform), With<Pet>>,
) {
    for (mut pet, mut transform) in &mut q {
        if pet.type_ != 1 {
            continue;
        }
        let mut idx = 0;
        unsafe { PetSpeed += (1. - PetSpeed) / 10. }
        for state in pet.state.clone() {
            match state {
                State::Jump => {
                    unsafe {
                        PetPos.y -= 100.;
                        PetSpeed = 10.;
                    }
                    pet.state.remove(idx as usize);
                    idx -= 1;
                }
                State::Left => unsafe { PetPos.x -= PetSpeed },

                State::Right => unsafe { PetPos.x += PetSpeed },
                _ => {}
            }
            idx += 1;
        }
    }
}

fn gravity(
    windows: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut q: Query<(&mut Pet, &mut Transform), With<Pet>>,
) {
    let mut monitor = windows.single();

    for (mut pet, mut transform) in &mut q {
        // commands.entity(entity)
        if pet.type_ != 1 {
            continue;
        }
        unsafe {
            PetPos.y += 3.;
        }
    }
}

fn manage_state(mut q: Query<&mut Pet, With<Pet>>, time: Res<Time>) {
    for mut pet in &mut q {
        if pet.type_ != 1 {
            continue;
        }
        pet.timer.tick(time.delta());

        if pet.timer.finished() {
            let mut rng = rand::thread_rng();
            if rng.gen_range(0..=3) == 0 {
                if pet.state.length() > 0 {
                    pet.state.pop();
                }
            } else {
                let s = State::int2state(rng.gen_range(0..=1));
                if let Some(old_sate) = pet.state.pop() {
                    log::info!("{:?} {:?}", old_sate, s);
                    if old_sate == s {
                        pet.state.push(State::Jump);
                    }
                }
                pet.state.push(s);
            }
            log::info!("{:?}", pet.state);
        }
    }
}

// 박스 밖으로 나가면 강재로 넣어놓음
fn boxin(
    windows: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut q: Query<(&mut Pet, &mut Transform), With<Pet>>,
) {
    let mut monitor = windows.single();

    for (mut pet, mut transform) in &mut q {
        // commands.entity(entity)
        if pet.type_ != 1 {
            continue;
        }

        if unsafe { PetPos.y } > monitor.height() as f32 - pet.size.y {
            unsafe { PetPos.y = monitor.height() - pet.size.y }
        }
        if unsafe { PetPos.y } < 0. {
            unsafe { PetPos.y = 0. }
        }
        if unsafe { PetPos.x } < 0. {
            unsafe {
                PetPos.x = 0.;
                PetPos.y -= 3.5;
            }
        }
        if unsafe { PetPos.x } > monitor.width() as f32 - pet.size.x {
            unsafe {
                PetPos.x = monitor.width() as f32 - pet.size.x;
                PetPos.y -= 3.5;
            }
        }
    }
}
