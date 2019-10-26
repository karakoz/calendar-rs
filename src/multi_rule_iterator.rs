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
    pub inner_iterators: Option<Vec<RuleIterator<'a>>>,
    cur_iterator_index: usize
}

impl<'a> MultiRuleIterator<'a> {
    pub fn new(rules: &'a Vec<Rule>, 
        cycle_start: NaiveDateTime,
        start_offset: NaiveDateTime, 
        end: NaiveDateTime
    ) -> Self {
        MultiRuleIterator {
            rules: rules,
            from: start_offset,
            to: end,
            cycle_start,
            cur_date: None,
            inner_iterators: None,
            cur_iterator_index: 0
        }
    }
}

impl<'a> Iterator for MultiRuleIterator<'a> {
    type Item = TimeInterval; 

    fn next(&mut self) -> Option<TimeInterval> {

        if let None = self.inner_iterators {
            let mut iters = Vec::<RuleIterator>::new();
            // let rule = &self.rules[0];

            for r in self.rules {
                let iter = r.get_iterator(self.cycle_start, self.from, self.to);
                iters.push(iter);
            }

            // let mut iter = rule.get_iterator(self.cycle_start, self.from, self.to);
            
            // let next = iter.next();

            // iters.push(iter);

            self.inner_iterators = Some(iters);
            self.cur_iterator_index = 0;

            // return next;
        }

        if let Some(inner) = &mut self.inner_iterators {
            loop {
                // if inner.len() <= self.cur_iterator_index {
                //     self.inner_iterators = None;
                //     self.cur_iterator_index = 0;
                //     return None;
                // }

                let fi = &mut inner[self.cur_iterator_index];

                let res = fi.next();

                if let None = res {
                    self.cur_iterator_index += 1;
                    continue;
                }

                return res;
            }
        };

        None
    }
}