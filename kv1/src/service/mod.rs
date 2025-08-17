mod command_service;

use crate::*;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}
