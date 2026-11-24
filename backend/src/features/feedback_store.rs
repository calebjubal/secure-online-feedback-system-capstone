use std::sync::{Arc, RwLock};

use serde::Serialize;
use uuid::Uuid;

use super::feedback::{FeedbackCategory, ValidatedFeedback};

pub const MAX_STORED_RECORDS: usize = 10_000;

/// Development-only storage. Replace this with a reviewed PostgreSQL-backed
/// repository before deploying or collecting real feedback.
#[derive(Clone, Default)]
pub struct FeedbackStore {
    entries: Arc<RwLock<Vec<FeedbackRecord>>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FeedbackRecord {
    pub id: Uuid,
    pub category: FeedbackCategory,
    pub message: String,
}

impl FeedbackStore {
    pub fn submit(&self, feedback: ValidatedFeedback) -> FeedbackRecord {
        let record = FeedbackRecord {
            id: Uuid::new_v4(),
            category: feedback.category,
            message: feedback.message,
        };
        let mut entries = self
            .entries
            .write()
            .expect("feedback store lock should not be poisoned");
        entries.push(record.clone());
        if entries.len() > MAX_STORED_RECORDS {
            let expired = entries.len() - MAX_STORED_RECORDS;
            entries.drain(..expired);
        }
        record
    }

    pub fn list_page(&self, offset: usize, limit: usize) -> Vec<FeedbackRecord> {
        self.entries
            .read()
            .expect("feedback store lock should not be poisoned")
            .iter()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_a_submitted_feedback_record() {
        let store = FeedbackStore::default();
        let saved = store.submit(ValidatedFeedback {
            category: FeedbackCategory::Facilities,
            message: "The library lighting could be improved.".into(),
        });

        let entries = store.list_page(0, 1);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, saved.id);
    }

    #[test]
    fn evicts_oldest_records_at_the_memory_bound() {
        let store = FeedbackStore::default();
        let first = store.submit(ValidatedFeedback {
            category: FeedbackCategory::Other,
            message: "first".into(),
        });
        for index in 1..=MAX_STORED_RECORDS {
            store.submit(ValidatedFeedback {
                category: FeedbackCategory::Other,
                message: index.to_string(),
            });
        }

        let entries = store.list_page(0, MAX_STORED_RECORDS);
        assert_eq!(entries.len(), MAX_STORED_RECORDS);
        assert!(!entries.iter().any(|entry| entry.id == first.id));
    }

    #[test]
    fn returns_only_the_requested_page() {
        let store = FeedbackStore::default();
        for message in ["first", "second", "third"] {
            store.submit(ValidatedFeedback {
                category: FeedbackCategory::Other,
                message: message.into(),
            });
        }

        let page = store.list_page(1, 1);
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].message, "second");
    }
}
