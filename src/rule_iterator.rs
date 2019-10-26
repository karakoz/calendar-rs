use crate::multi_rule_iterator::MultiRuleIterator;
use crate::time_interval::TimeInterval;
use crate::rule::Rule;
use chrono::{NaiveDateTime, Duration};
use crate::repetition_kind::RepetitionKind;
use crate::offset_kind::OffsetKind;


pub struct RuleIterator<'a> {
    pub rule: &'a Rule,
    pub from: NaiveDateTime, // todo: may be duration
    pub to: NaiveDateTime, // Option<NaiveDateTime>,
    pub cur_offset: Option<NaiveDateTime>,
    pub cycle_start: NaiveDateTime,
    pub inner_iterator: Option<MultiRuleIterator<'a>>
}

impl<'a> Iterator for RuleIterator<'a> {
    type Item = TimeInterval; 

    fn next(&mut self) -> Option<TimeInterval> {

        if let Some(inner) = &mut self.inner_iterator {
            let next = inner.next();

            if let Some(n) = next {
                return Some(n)
            } else {
            }
        }

        self.inner_iterator = None;

        if let Some(repetition) = &self.rule.repetition {
            match repetition {
                RepetitionKind::Duration(rep_dur) => {
                    let start;
                    let end;
                    let cur_offset;
                    if let Some(co) = self.cur_offset {
                        cur_offset = co + *rep_dur;

                        start = cur_offset;
    
                        end = if self.to < start + self.rule.length {
                            self.to
                        } else {
                            start + self.rule.length
                        }
                    } else {
                        match self.rule.offset {
                            OffsetKind::DateTime(offset) => {
                                let co = get_closest_offset_to_left(offset, *rep_dur, self.from);

                                if co + self.rule.length <= self.from {
                                    cur_offset = co + *rep_dur;
                                } else {
                                    cur_offset = co;
                                }

                                start = if self.from > cur_offset {
                                                self.from
                                            } else {
                                                cur_offset
                                            };
                                end = if self.to < cur_offset + self.rule.length {
                                                self.to
                                            } else {
                                                cur_offset + self.rule.length
                                            };
                              
                            },
                            OffsetKind::Duration(offset_dur) => {
                                let co = get_closest_offset_to_left(self.cycle_start + offset_dur, *rep_dur, self.from);

                                if co + self.rule.length <= self.from {
                                    cur_offset = co + *rep_dur;
                                } else {
                                    cur_offset = co;
                                }


                                start = if self.from > cur_offset { self.from } else { cur_offset };

                                end = if self.to < cur_offset + self.rule.length { self.to } else { cur_offset + self.rule.length }; 
                            }
                        }
                    }

                    self.cur_offset = Some(cur_offset);

                    if let Some(inner_rules) = &self.rule.inner_rules {

                        self.inner_iterator = Some(get_inner_iterator(inner_rules, cur_offset, start, end));

                        return self.next()
                    } else {
                        return Some(TimeInterval { start, end })
                    }
                },
                _ => ()
            }
        }

        None
    } 
}

fn get_inner_iterator(
    inner_rules: &Vec<Rule>, 
    cycle_start: NaiveDateTime,
    start_offset: NaiveDateTime, 
    end: NaiveDateTime) 
        -> MultiRuleIterator {

    MultiRuleIterator::new(
        inner_rules,
        cycle_start,
        start_offset,
        end)
}

fn get_closest_offset_to_left(offset: NaiveDateTime, repetition: Duration, target: NaiveDateTime) -> NaiveDateTime {
    let sub = target - offset;

    let div = sub.num_seconds() / repetition.num_seconds();

    offset + repetition * (div as i32)
}

