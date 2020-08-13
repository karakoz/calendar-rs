use calendar::union_iterator::UnionIterator;
use calendar::time_interval::TimeInterval;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[test]
fn union_works() {
    let date = NaiveDate::from_ymd(2020, 1, 10);

    let v1 = vec![
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(10, 10, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 10, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(10, 20, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 12, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(10, 25, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 30, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(10, 40, 0)),
            },
        ]
        .into_iter();

    let v2 = vec![
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(9, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(9, 10, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 10, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(10, 22, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 28, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(10, 40, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(11, 00, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(12, 00, 0)),
            },
        ]
        .into_iter();
    
    let union = UnionIterator::new(vec![v1, v2]);
    let res: Vec<TimeInterval> = union.collect::<Vec<_>>();

    assert_eq!(
        res,
        vec![
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(9, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(9, 10, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 0, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(10, 25, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(10, 28, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(10, 40, 0)),
            },
            TimeInterval {
                start: NaiveDateTime::new(date, NaiveTime::from_hms(11, 00, 0)),
                end: NaiveDateTime::new(date, NaiveTime::from_hms(12, 00, 0)),
            },
        ]
    );
}
