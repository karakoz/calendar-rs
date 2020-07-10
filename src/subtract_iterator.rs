use crate::time_interval::TimeInterval;

pub struct SubtractIterator<I: Iterator<Item = TimeInterval>> { 
    source: I,
    subtrahend: I,
    cur: Option<(Option<TimeInterval>, Option<TimeInterval>)>,
}

impl<I: Iterator<Item = TimeInterval>> SubtractIterator<I> {
    pub fn new(source: I, subtrahend: I) -> Self {
        SubtractIterator {
            source,
            subtrahend,
            cur: None
        }
    }
}

impl<I: Iterator<Item = TimeInterval>> Iterator for SubtractIterator<I> {
    type Item = TimeInterval;

    fn next(&mut self) -> Option<TimeInterval> { 
        if self.cur.is_none() {
            self.cur = Some((self.source.next(), self.subtrahend.next()));
        }

        let cur = self.cur.as_mut().unwrap();
        
        loop {
        
            if cur.0.is_none() {
                return None
            }
    
            let mut src = cur.0.unwrap();
    
            if cur.1.is_none() {
                cur.0 = self.source.next();
                return Some(src);
            }

            let mut sub: TimeInterval;
            loop {
                if cur.1.is_none() {
                    cur.0 = self.source.next();
                    return Some(src);
                }

                sub = cur.1.unwrap();

                // a.1
                if sub.end <= src.start {
                    cur.1 = self.subtrahend.next();
                    continue;
                }
                break;
            }

            // a.4
            if sub.start >= src.end {
                cur.0 = self.source.next();
                return Some(src);
            } 

            // a.3, b.1
            if sub.start > src.start {

                // a.3
                if sub.end < src.end {
                    cur.0.as_mut().unwrap().start = sub.end;
                    cur.1 = self.subtrahend.next();
                } else { 
                    // b.1
                    cur.0 = self.source.next();
                };

                return Some(TimeInterval {
                    start: src.start,
                    end: sub.start
                })
            }

            // c.1
            if sub.end >= src.end {
                cur.0 = self.source.next();
                continue;
            }

            // a.2
            cur.0.as_mut().unwrap().start = sub.end;
        }
    }
}