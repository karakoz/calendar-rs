use crate::rule_item::RuleItem;
use crate::rule::Rule;
use crate::multi_rule_iterator::MultiRuleIterator;
use crate::time_interval::TimeInterval;
use chrono::{NaiveDateTime, Datelike, Duration};
use crate::repetition_kind::RepetitionKind;
use crate::offset_kind::OffsetKind;

pub struct RuleIterator<'a, V: Sized> {
    pub rule: &'a Rule<V>,
    pub from: NaiveDateTime, // todo: may be duration
    pub to: NaiveDateTime, // Option<NaiveDateTime>,
    pub cur_offset: Option<NaiveDateTime>,
    pub cycle_start: NaiveDateTime,
    pub inner_iterator: Option<MultiRuleIterator<'a, V>>
}

impl<'a, V: Sized> Iterator for RuleIterator<'a, V> {
    type Item = (TimeInterval, &'a RuleItem); 

    fn next(&mut self) -> Option<(TimeInterval, &'a RuleItem)> {

        if let Some(inner) = &mut self.inner_iterator {
            let next = inner.next();

            if let Some(n) = next {
                return Some(n)
            } else {
            }
        }

        self.inner_iterator = None;

        if let Some(repetition) = &self.rule.rule_item.repetition {
            let start;
            let end;
            let cur_offset;
            if let Some(co) = self.cur_offset {
                cur_offset = add_repetition(co, repetition);

                start = cur_offset;

                end = if self.to < start + self.rule.rule_item.length {
                    self.to
                } else {
                    start + self.rule.rule_item.length
                };

                if start >= end {
                    return None;
                }
            } else {
                let offset = 
                    match self.rule.rule_item.offset {
                        OffsetKind::DateTime(offset) => offset,
                        OffsetKind::Duration(offset_dur) => self.cycle_start + offset_dur
                    };

                let co = get_closest_offset(offset, repetition, self.from);

                if co + self.rule.rule_item.length <= self.from {
                    cur_offset = add_repetition(co, repetition);
                } else {
                    cur_offset = co;
                }

                start = if self.from > cur_offset {
                                self.from
                            } else {
                                cur_offset
                            };
                end = if self.to < cur_offset + self.rule.rule_item.length {
                                self.to
                            } else {
                                cur_offset + self.rule.rule_item.length
                            };
            }

            self.cur_offset = Some(cur_offset);

            if let Some(inner_rules) = &self.rule.inner_rules {

                self.inner_iterator = Some(get_inner_iterator(inner_rules, cur_offset, start, end));

                return self.next()
            } else {
                return Some((TimeInterval { start, end }, &self.rule.rule_item))
            }
        } else {
            if let Some(_) = self.cur_offset {
                return None;
            }

            match self.rule.rule_item.offset {
                OffsetKind::DateTime(_) => {
                    unimplemented!()
                },
                OffsetKind::Duration(offset_dur) => {
                    let cur_offset = self.cycle_start + offset_dur;

                    let start = if self.from > cur_offset { self.from } else { cur_offset };
                    let end = if self.to < cur_offset + self.rule.rule_item.length { self.to } 
                            else { cur_offset + self.rule.rule_item.length }; 
                
                    self.cur_offset = Some(cur_offset);

                    if let Some(inner_rules) = &self.rule.inner_rules {

                        self.inner_iterator = Some(get_inner_iterator(inner_rules, cur_offset, start, end));

                        return self.next()
                    } else {
                        return Some((TimeInterval { start, end }, &self.rule.rule_item))
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

fn get_inner_iterator<'a, V: Sized>(
    inner_rules: &'a Vec<Rule<V>>, 
    cycle_start: NaiveDateTime,
    start_offset: NaiveDateTime, 
    end: NaiveDateTime) 
        -> MultiRuleIterator<'a, V> {

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

