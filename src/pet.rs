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
    type_: bool,
}
impl Pet {
    pub fn new(type_: bool) -> Self {
        Self {
            state: vec![State::Jump],
            pos: Vec2::new(300., 10000.),
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
            .add_systems(Update, movement);
    }
}

pub fn change_info(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut q: Query<(&Pet, Entity, &mut Style), With<Pet>>,
) {
    for (pet, entity, mut style) in &mut q {
        let mut component = commands.entity(entity);
        if let (Val::Px(top), Val::Px(left)) = (style.top, style.left) {
            style.top = Val::Px(top + (pet.pos.y - top) / 10.);
            style.left = Val::Px(left + (pet.pos.x - left) / 10.);
            style.width = Val::Px(pet.size.x);
            style.height = Val::Px(pet.size.y);
        } else {
            style.top = Val::Px(pet.pos.y);
            style.left = Val::Px(pet.pos.x);
        }

        if pet.type_ {
            //
        }
    }
}

fn movement(
    windows: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut q: Query<(&mut Pet, &mut Transform), With<Pet>>,
) {
    for (mut pet, mut transform) in &mut q {
        let mut idx = 0;
        for state in pet.state.clone() {
            match state {
                State::Jump => {
                    pet.pos.y -= 100.;
                    pet.state.remove(idx as usize);
                    idx -= 1;
                }
                State::Left => pet.pos.x -= 1.,

                State::Right => pet.pos.x += 1.,
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

        pet.pos.y += 3.;
    }
}

fn manage_state(mut q: Query<&mut Pet, With<Pet>>, time: Res<Time>) {
    for mut pet in &mut q {
        pet.timer.tick(time.delta());

        if pet.timer.finished() {
            let mut rng = rand::thread_rng();
            if rng.gen_range(0..=3) == 0 {
                if pet.state.length() > 0 {
                    pet.state.pop();
                }
            } else {
                let s = State::int2state(rng.gen_range(0..=2));
                if !pet.state.contains(&s) {
                    pet.state.push(s);
                }
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

        if pet.pos.y > monitor.height() as f32 - pet.size.y {
            pet.pos.y = monitor.height() - pet.size.y
        }
        if pet.pos.y < 0. {
            pet.pos.y = 0.
        }
        if pet.pos.x < 0. {
            pet.pos.x = 0.
        }
        if pet.pos.x > monitor.width() as f32 - pet.size.x {
            pet.pos.x = monitor.width() as f32 - pet.size.x
        }
    }
}
