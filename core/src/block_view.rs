use crate::pb::sf::near::r#type::v1 as pb;
use hex;

impl pb::Block {
    pub fn state_changes(&self) -> StateChangesView<'_> {
        StateChangesView {
            state_changes: &self.state_changes,
        }
    }

    pub fn hash(&self) -> Option<String> {
        self.header
            .as_option()
            .and_then(|header| header.hash.as_option())
            .map(|hash| hex::encode(&hash.bytes))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_none_without_a_header() {
        assert_eq!(pb::Block::default().hash(), None);
    }

    #[test]
    fn hash_is_none_when_the_header_carries_no_hash() {
        let mut block = pb::Block::default();
        block.header = buffa::MessageField::some(pb::BlockHeader::default());

        assert_eq!(block.hash(), None);
    }

    #[test]
    fn hash_is_hex_encoded() {
        let mut header = pb::BlockHeader::default();
        header.hash = buffa::MessageField::some(pb::CryptoHash {
            bytes: vec![0xde, 0xad, 0xbe, 0xef],
            ..Default::default()
        });

        let mut block = pb::Block::default();
        block.header = buffa::MessageField::some(header);

        assert_eq!(block.hash(), Some("deadbeef".to_string()));
    }
}
