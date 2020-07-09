use crate::time_interval::TimeInterval;
use std::cmp::Ordering;

pub struct UnionIterator<I: Iterator<Item = TimeInterval>> { 
    iterators: Vec<I>,
    cur: Vec<Option<TimeInterval>>,
    cur_index: usize,
    is_init: bool,
}

impl<I: Iterator<Item = TimeInterval>> UnionIterator<I> {
    pub fn new(iterators: Vec<I>) -> Self {
        let len = iterators.len();
        UnionIterator {
            iterators,
            cur: vec![None; len],
            cur_index: 0,
            is_init: false,
        }
    }

    fn init_all(&mut self) {
        let mut i = 0;
        let mut min: Option<TimeInterval> = None;
        let mut min_index = 0;
        for iterator in self.iterators.iter_mut() {
            let next = iterator.next();
            self.cur[i] = next;

            if let Some(next) = next {
                if min.is_none() {
                    min = Some(next);
                    min_index = i;
                } else {
                    match min.unwrap().start.cmp(&next.start) {
                        Ordering::Greater => { 
                            min = Some(next);
                            min_index = i;
                        },
                        _ => ()
                    }
                }

            }
            i += 1;
        }
        self.is_init = true;
        self.cur_index = min_index;
    }

    fn get_min_pos(&self) -> Option<usize> {
        
        let mut i = 0;
        let mut min: Option<&TimeInterval> = None;
        let mut min_index: usize = 0;
        for elem in self.cur.iter() {
            if let Some(next) = elem {
                if min.is_none() {
                    min = Some(next);
                    min_index = i;
                } else {
                    match min.unwrap().start.cmp(&next.start) {
                        Ordering::Greater => { 
                            min = Some(next);
                            min_index = i;
                        },
                        _ => ()
                    }
                }
            }
            i += 1;
        }
        if min.is_none() { 
            return None
        }
        Some(min_index)
    }

    fn update_cur_value(&mut self) {
        self.cur[self.cur_index] = self.iterators[self.cur_index].next();
    }
}

impl<I: Iterator<Item = TimeInterval>> Iterator for UnionIterator<I> {
    type Item = TimeInterval;

    fn next(&mut self) -> Option<TimeInterval> {
        if !self.is_init {
            self.init_all();
        } 

        let len = self.cur.len();

        let min = self.get_min_pos();
        if min.is_none() {
            return None;
        }

        self.cur_index = min.unwrap();
        let next = self.cur[self.cur_index];
        self.update_cur_value();

        let mut next = next.unwrap();

        let mut c = 0;
        loop {
            
            loop {
                let cur = self.cur[self.cur_index]; 
                
                if cur.is_none() {
                    c += 1;
                    break;
                }

                let cur = cur.unwrap();
                
                if cur.start > next.end {
                    c += 1;
                    break;
                }

                if cur.end > next.end {
                    next.end = cur.end;
                }
                self.update_cur_value();
                c = 0;
            }

            if c == len {
                return Some(next);
            }

            self.cur_index = (self.cur_index + 1) % len;
        }
    } 
}
