use crate::rule_item::RuleItem;
use crate::rule::Rule;
use crate::rule_iterator::RuleIterator;
use crate::time_interval::TimeInterval;
use chrono::NaiveDateTime;

pub struct MultiRuleIterator<'a, V: Sized> {
    pub rules: &'a Vec<Rule<V>>,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub cycle_start: NaiveDateTime,
    pub cur_date: Option<NaiveDateTime>,
    pub inner_iterators: Option<Vec<RuleIterator<'a, V>>>,
    cur_iterator_index: usize
}

impl<'a, V: Sized> MultiRuleIterator<'a, V> {
    pub fn new(rules: &'a Vec<Rule<V>>, 
        cycle_start: NaiveDateTime,
        start_offset: NaiveDateTime, 
        end: NaiveDateTime,
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

impl<'a, V: Sized> Iterator for MultiRuleIterator<'a, V> {
    type Item = (TimeInterval, &'a RuleItem); 

    fn next(&mut self) -> Option<(TimeInterval, &'a RuleItem)> {

        if let None = self.inner_iterators {
            let mut iters = Vec::<RuleIterator<V>>::new();
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
                if inner.len() <= self.cur_iterator_index {
                    self.inner_iterators = None;
                    self.cur_iterator_index = 0;
                    return None;
                }

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