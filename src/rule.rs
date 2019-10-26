use crate::rule_iterator::RuleIterator;
use chrono::NaiveDateTime;
use crate::offset_kind::OffsetKind;
use crate::repetition_kind::RepetitionKind;
use chrono::Duration;

#[derive(Clone)]
pub struct Rule {
    pub offset: OffsetKind,
    pub repetition: Option<RepetitionKind>,
    pub length: Duration,
    pub inner_rules: Option<Vec<Rule>>,
}

impl Rule {
    pub fn get_iterator(&self, cycle_start: NaiveDateTime, from: NaiveDateTime, to: NaiveDateTime) -> RuleIterator {
        RuleIterator {
            rule: self,
            from,
            to, 
            cur_offset: None,
            cycle_start,
            inner_iterator: None
        }
    }
}
