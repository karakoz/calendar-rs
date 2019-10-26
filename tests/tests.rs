mod helper;
use helper::dt_from_ymd_hms;

use calendar::rule::Rule;
use calendar::offset_kind::OffsetKind;
use calendar::repetition_kind::RepetitionKind;
use calendar::time_interval::TimeInterval;
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};


#[test]
fn rule_works() {
    let eight_hour_day_rule = Rule {
        offset: OffsetKind::Duration(Duration::hours(9) + Duration::minutes(15)),
        repetition: Some(RepetitionKind::Duration(Duration::days(1))),
        length: Duration::hours(8) + Duration::minutes(45),
        inner_rules: None
    };

    let inner_rules = vec![eight_hour_day_rule; 1];

    let rule = Rule {
        offset: OffsetKind::DateTime(NaiveDateTime::new(NaiveDate::from_ymd(2001, 1, 1), NaiveTime::from_hms(0, 0, 0))),
        repetition: Some(RepetitionKind::Duration(Duration::days(7))),
        length: Duration::days(5),
        inner_rules: Some(inner_rules)
    };
    
    let iter = rule.get_iterator(
        dt_from_ymd_hms(2001, 1, 1, 0, 0, 0),
        dt_from_ymd_hms(2019, 1, 30, 0, 0, 0), 
        dt_from_ymd_hms(2019, 2, 22, 17, 17, 0));

    let intervals: Vec<TimeInterval> = iter.take(30).collect();

    let vec = vec![
            TimeInterval {
                start: dt_from_ymd_hms(2019, 1, 30, 9, 15, 0), 
                end: dt_from_ymd_hms(2019, 1, 30, 18, 00, 0)
            },
            TimeInterval {
                start: dt_from_ymd_hms(2019, 1, 31, 9, 15, 0), 
                end: dt_from_ymd_hms(2019, 1, 31, 18, 00, 0)
            },
            TimeInterval {
                start: dt_from_ymd_hms(2019, 2, 1, 9, 15, 0), 
                end: dt_from_ymd_hms(2019, 2, 1, 18, 00, 0)
            },
        ];

    for i in vec![0, 1, 2] {
        assert_eq!(intervals[i].start, vec[i].start);
        assert_eq!(intervals[i].end, vec[i].end);
    }

    let little_interval: Vec<TimeInterval> = rule.get_iterator(
        dt_from_ymd_hms(2001, 1, 1,  0, 0, 0),
        dt_from_ymd_hms(2019, 1, 30, 10, 10, 0), 
        dt_from_ymd_hms(2019, 1, 30, 11, 11, 0)).take(1).collect();

    assert_eq!(little_interval[0].start, dt_from_ymd_hms(2019, 1, 30, 10, 10, 0));
    assert_eq!(little_interval[0].end, dt_from_ymd_hms(2019, 1, 30, 11, 11, 0));

}