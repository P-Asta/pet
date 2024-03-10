#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Left,
    Right,
    Jump,
    Clim,
}
impl State {
    pub fn int2state(i: usize) -> Self {
        match i {
            0 => State::Left,
            1 => State::Right,
            2 => State::Jump,
            _ => State::Clim,
        }
    }
}
