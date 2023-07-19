use crate::{pb::sf::near::r#type::v1 as pb};
use hex;

impl pb::Block {
    pub fn state_changes(&self) -> StateChangesView<'_> {
        StateChangesView {
            state_changes: &self.state_changes,
        }
    }

    pub fn hash(&self) -> Option<String> {
        match &self.header {
            Some(header) => {
                match &header.hash {
                    Some(hash) => Some(hex::encode(&hash.bytes)),
                    None => None,
                }
            },
            None => None,
        }
    }
}

pub struct StateChangesView<'a> {
    pub state_changes: &'a Vec<pb::StateChangeWithCause>,
}

impl AsRef<Vec<pb::StateChangeWithCause>> for StateChangesView<'_> {
    fn as_ref(&self) -> &Vec<pb::StateChangeWithCause> {
        self.state_changes
    }
}

