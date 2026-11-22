use std::sync::{Arc, RwLock};

use serde::Serialize;
use uuid::Uuid;

use super::feedback::{FeedbackCategory, ValidatedFeedback};

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
        self.entries
            .write()
            .expect("feedback store lock should not be poisoned")
            .push(record.clone());
        record
    }

    pub fn list(&self) -> Vec<FeedbackRecord> {
        self.entries
            .read()
            .expect("feedback store lock should not be poisoned")
            .clone()
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

        assert_eq!(store.list().len(), 1);
        assert_eq!(store.list()[0].id, saved.id);
    }
}
