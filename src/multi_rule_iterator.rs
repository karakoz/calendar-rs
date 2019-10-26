use crate::rule_iterator::RuleIterator;
use crate::time_interval::TimeInterval;
use crate::rule::Rule;
use chrono::NaiveDateTime;

pub struct MultiRuleIterator<'a> {
    pub rules: &'a Vec<Rule>,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub cycle_start: NaiveDateTime,
    pub cur_date: Option<NaiveDateTime>,
    pub inner_iterators: Option<Vec<RuleIterator<'a>>>
}

impl<'a> Iterator for MultiRuleIterator<'a> {
    type Item = TimeInterval; 

    fn next(&mut self) -> Option<TimeInterval> {

        if let None = self.inner_iterators {
            let mut iters = Vec::<RuleIterator>::new();
            let rule = &self.rules[0];

            let mut iter = rule.get_iterator(self.cycle_start, self.from, self.to);
            
            let next = iter.next();

            iters.push(iter);

            self.inner_iterators = Some(iters);

            return next;
        }

        if let Some(inner) = &mut self.inner_iterators {
            let fi = &mut inner[0];

            let res = fi.next();

            return res;
        };

        None
    }
}