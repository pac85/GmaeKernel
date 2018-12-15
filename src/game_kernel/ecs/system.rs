use super::super::settings::Config;

use std::fmt;

struct SysErr
{

}

impl SysErr
{
    pub fn new() -> Self
    {
        Self{}
    }
}

impl fmt::Display for SysErr
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

struct SystemDeps
{

}

trait System
{
    fn get_deps() -> &'static SystemDeps;
    fn init(config: &mut Config) -> Result<(), SysErr>;
}

pub struct SystemManager
{

}