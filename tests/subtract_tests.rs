use calendar::subtract_iterator::SubtractIterator;
use calendar::time_interval::TimeInterval;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[test]
fn subtract_works() {
    let date = NaiveDate::from_ymd(2020, 1, 10);

    let source = vec![
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(11, 0, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(12, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(14, 0, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(16, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(20, 0, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(21, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(22, 0, 0)),
            },
        ]
        .into_iter();

    let subtrahend = vec![
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(9, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(9, 10, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(10, 10, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 30, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(10, 40, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(12, 30, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(13, 0, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(13, 10, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(14, 40, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(15, 10, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(20, 40, 0)),
            },
            
        ]
        .into_iter();
    
    let sub = SubtractIterator::new(source, subtrahend);
    let res: Vec<TimeInterval> = sub.collect::<Vec<_>>();

    assert_eq!(
        res,
        vec![
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 10, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(10, 30, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 40, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(11, 0, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(12, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(12, 30, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(13, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(13, 10, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(21, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(22, 0, 0)),
            },
        ]
    );
}
