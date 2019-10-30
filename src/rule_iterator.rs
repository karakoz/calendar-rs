use crate::multi_rule_iterator::MultiRuleIterator;
use crate::time_interval::TimeInterval;
use crate::rule::Rule;
use chrono::{NaiveDateTime, Datelike, Duration};
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
            let start;
            let end;
            let cur_offset;
            if let Some(co) = self.cur_offset {
                cur_offset = add_repetition(co, repetition);// co + *rep_dur;

                start = cur_offset;

                end = if self.to < start + self.rule.length {
                    self.to
                } else {
                    start + self.rule.length
                };

                if start >= end {
                    return None;
                }
            } else {
                let offset = 
                    match self.rule.offset {
                        OffsetKind::DateTime(offset) => offset,
                        OffsetKind::Duration(offset_dur) => self.cycle_start + offset_dur
                    };

                let co = get_closest_offset(offset, repetition, self.from);

                if co + self.rule.length <= self.from {
                    cur_offset = add_repetition(co, repetition); // co + *rep_dur;
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
            }

            self.cur_offset = Some(cur_offset);

            if let Some(inner_rules) = &self.rule.inner_rules {

                self.inner_iterator = Some(get_inner_iterator(inner_rules, cur_offset, start, end));

                return self.next()
            } else {
                return Some(TimeInterval { start, end })
            }
        } else {
            if let Some(_) = self.cur_offset {
                return None;
            }

            match self.rule.offset {
                OffsetKind::DateTime(_) => {
                    unimplemented!()
                },
                OffsetKind::Duration(offset_dur) => {
                    let cur_offset = self.cycle_start + offset_dur;

                    let start = if self.from > cur_offset { self.from } else { cur_offset };
                    let end = if self.to < cur_offset + self.rule.length { self.to } else { cur_offset + self.rule.length }; 
                
                    self.cur_offset = Some(cur_offset);

                    if let Some(inner_rules) = &self.rule.inner_rules {

                        self.inner_iterator = Some(get_inner_iterator(inner_rules, cur_offset, start, end));

                        return self.next()
                    } else {
                        return Some(TimeInterval { start, end })
                    }
                }
            }
        }
    } 
}

fn add_repetition(source: NaiveDateTime, rep: &RepetitionKind) -> NaiveDateTime {
    match rep {
        RepetitionKind::Duration(dur) => source + *dur,
        RepetitionKind::Years(y) => source.with_year(source.year() + *y as i32).unwrap(),
        _ => unimplemented!()
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

fn get_closest_offset(start: NaiveDateTime, repetition: &RepetitionKind, target: NaiveDateTime) -> NaiveDateTime {
    match repetition {
        RepetitionKind::Duration(repetition) => {
            let sub = target - start;

            let div = sub.num_seconds() / repetition.num_seconds();

            start + *repetition * (div as i32)
        },
        RepetitionKind::Years(y) => {
            let sub = target - start;            
            // todo: this is a rough realization
            let div = sub.num_seconds() / Duration::days(365 * *y as i64).num_seconds();

            start.with_year(start.year() + (*y as i32) * div as i32).unwrap()
        },
        _ => {
            unimplemented!()
        }
    }
}

