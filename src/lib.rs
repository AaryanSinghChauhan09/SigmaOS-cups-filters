// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-cups-filters: cups filters component model.
//!
//! Real implementation: filters, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filter {
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
pub enum CupsFiltersError {
    EmptyFilterName,
    DuplicateFilter(String),
    EmptyEventSubject,
}

impl fmt::Display for CupsFiltersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFilterName => write!(f, "filter name must not be empty"),
            Self::DuplicateFilter(id) => write!(f, "duplicate filter {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for CupsFiltersError {}

pub struct CupsFilters {
    pub name: String,
    filters: BTreeMap<String, Filter>,
    events: BTreeMap<String, Event>,
}

impl CupsFilters {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            filters: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_filter(&mut self, item: Filter) -> Result<(), CupsFiltersError> {
        if item.name.trim().is_empty() {
            return Err(CupsFiltersError::EmptyFilterName);
        }
        let id = item.id.clone();
        if self.filters.contains_key(&id) {
            return Err(CupsFiltersError::DuplicateFilter(id.clone()));
        }
        self.filters.insert(id, item);
        Ok(())
    }

    pub fn filter_count(&self) -> usize {
        self.filters.len()
    }

    pub fn get_filter(&self, id: &str) -> Option<&Filter> {
        self.filters.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), CupsFiltersError> {
        if event.subject.trim().is_empty() {
            return Err(CupsFiltersError::EmptyEventSubject);
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

    fn sample_filter(id: &str, name: &str) -> Filter {
        Filter {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = CupsFilters::new("CupsFilters");
        assert_eq!(m.name, "CupsFilters");
        assert_eq!(m.filter_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_filter() {
        let mut m = CupsFilters::new("CupsFilters");
        m.add_filter(sample_filter("a1", "Alpha")).unwrap();
        assert_eq!(m.filter_count(), 1);
        assert!(m.get_filter("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = CupsFilters::new("CupsFilters");
        assert!(m.add_filter(sample_filter("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = CupsFilters::new("CupsFilters");
        m.add_filter(sample_filter("x", "One")).unwrap();
        assert!(m.add_filter(sample_filter("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = CupsFilters::new("CupsFilters");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
