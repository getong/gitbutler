use std::time::SystemTime;

use super::{deltas, operations};

#[derive(Debug, Clone, Default)]
pub struct TextDocument {
    doc: Vec<char>,
    deltas: Vec<deltas::Delta>,
}

impl TextDocument {
    fn apply_deltas(doc: &mut Vec<char>, deltas: &Vec<deltas::Delta>) {
        if deltas.len() == 0 {
            return;
        }
        for event in deltas {
            for operation in event.operations.iter() {
                match operation {
                    operations::Operation::Insert((index, chunk)) => {
                        doc.splice(*index as usize..*index as usize, chunk.chars());
                    }
                    operations::Operation::Delete((index, len)) => {
                        doc.drain(*index as usize..(*index + *len) as usize);
                    }
                }
            }
        }
    }

    // creates a new text document from a deltas.
    pub fn from_deltas(deltas: Vec<deltas::Delta>) -> TextDocument {
        let mut doc = vec![];
        Self::apply_deltas(&mut doc, &deltas);
        TextDocument { doc, deltas }
    }

    pub fn get_deltas(&self) -> Vec<deltas::Delta> {
        self.deltas.clone()
    }

    // returns a text document where internal state is seeded with value, and deltas are applied.
    pub fn new(value: &str, deltas: Vec<deltas::Delta>) -> TextDocument {
        let mut all_deltas = vec![deltas::Delta {
            operations: operations::get_delta_operations("", value),
            timestamp_ms: 0,
        }];
        all_deltas.append(&mut deltas.clone());
        let mut doc = vec![];
        Self::apply_deltas(&mut doc, &all_deltas);
        TextDocument { doc, deltas }
    }

    pub fn update(&mut self, value: &str) -> bool {
        let diffs = operations::get_delta_operations(&self.to_string(), value);
        let event = deltas::Delta {
            operations: diffs,
            timestamp_ms: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        };
        if event.operations.len() == 0 {
            return false;
        }
        Self::apply_deltas(&mut self.doc, &vec![event.clone()]);
        self.deltas.push(event);
        return true;
    }

    pub fn to_string(&self) -> String {
        self.doc.iter().collect()
    }
}
