use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::Deref;
use parking_lot::Mutex;
use evmap;
use evmap::ShallowCopy;
pub use game_kernel_utils::hierarchy::*;

use super::component;

pub struct Entities{
    r: evmap::ReadHandle<Keytype, component::ComponentBox>,
    w: Mutex<evmap::WriteHandle<Keytype, component::ComponentBox>>,
}