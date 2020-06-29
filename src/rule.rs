use crate::rule_iterator::RuleIterator;
use chrono::NaiveDateTime;
use crate::rule_item::RuleItem;

#[derive(Clone)]
pub struct Rule<V: Sized> {
    pub rule_item: RuleItem,
    pub priority: i32,
    pub value: Option<V>,
    pub inner_rules: Option<Vec<Rule<V>>>,
}



impl<V: Sized> Rule<V> {
    pub fn get_iterator(&self, 
        cycle_start: NaiveDateTime, 
        from: NaiveDateTime, 
        to: NaiveDateTime) 
    -> RuleIterator<V>
    {
        RuleIterator {
            rule: self,
            from,
            to, 
            cur_offset: None,
            cycle_start,
            inner_iterator: None
        }
    }

    pub fn from_rule_item(rule_item: RuleItem, priority: i32, value: V, inner_rules: Option<Vec<Rule<V>>>) -> Self {
        Rule {
            rule_item: rule_item,
            priority,
            value: Some(value),
            inner_rules
        }
    }
}
