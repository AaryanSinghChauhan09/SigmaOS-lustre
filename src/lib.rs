// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-lustre: lustre component model.
//!
//! Real implementation: lustre_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LustreItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LustreError {
    EmptyLustreItemName,
    DuplicateLustreItem(String),
    EmptyEventSubject,
}

impl fmt::Display for LustreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLustreItemName => write!(f, "lustre_item name must not be empty"),
            Self::DuplicateLustreItem(id) => write!(f, "duplicate lustre_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LustreError {}

pub struct Lustre {
    pub name: String,
    lustre_items: BTreeMap<String, LustreItem>,
    events: BTreeMap<String, Event>,
}

impl Lustre {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            lustre_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_lustre_item(&mut self, item: LustreItem) -> Result<(), LustreError> {
        if item.name.trim().is_empty() {
            return Err(LustreError::EmptyLustreItemName);
        }
        let id = item.id.clone();
        if self.lustre_items.contains_key(&id) {
            return Err(LustreError::DuplicateLustreItem(id.clone()));
        }
        self.lustre_items.insert(id, item);
        Ok(())
    }

    pub fn lustre_item_count(&self) -> usize {
        self.lustre_items.len()
    }

    pub fn get_lustre_item(&self, id: &str) -> Option<&LustreItem> {
        self.lustre_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LustreError> {
        if event.subject.trim().is_empty() {
            return Err(LustreError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_lustre_item(id: &str, name: &str) -> LustreItem {
        LustreItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Lustre::new("Lustre");
        assert_eq!(m.name, "Lustre");
        assert_eq!(m.lustre_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_lustre_item() {
        let mut m = Lustre::new("Lustre");
        m.add_lustre_item(sample_lustre_item("a1", "Alpha")).unwrap();
        assert_eq!(m.lustre_item_count(), 1);
        assert!(m.get_lustre_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Lustre::new("Lustre");
        assert!(m.add_lustre_item(sample_lustre_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Lustre::new("Lustre");
        m.add_lustre_item(sample_lustre_item("x", "One")).unwrap();
        assert!(m.add_lustre_item(sample_lustre_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Lustre::new("Lustre");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
