mod helper;
use helper::dt_from_ymd_hms;

use calendar::rule::Rule;
use calendar::offset_kind::OffsetKind;
use calendar::repetition_kind::RepetitionKind;
use calendar::time_interval::TimeInterval;
use chrono::{Duration};


#[test]
fn rule_works() {
    // the work day starts at 9:15am with duration 8h 45min (ends at 6:00pm) 
    let eight_hour_day_rule = Rule {
        offset: OffsetKind::Duration(Duration::hours(9) + Duration::minutes(15)),
        repetition: Some(RepetitionKind::Duration(Duration::days(1))),
        length: Duration::hours(8) + Duration::minutes(45),
        inner_rules: None
    };

    let inner_rules = vec![eight_hour_day_rule; 1];

    // the rule starts at Monday 2001-1-1 at 12:00am, it repeats each 7 days, 
    // so it represents week rule starting from Monday with 5 working days.
    let rule = Rule {
        offset: OffsetKind::DateTime(dt_from_ymd_hms(2001, 1, 1, 0, 0, 0)),
        repetition: Some(RepetitionKind::Duration(Duration::days(7))),
        length: Duration::days(5),
        inner_rules: Some(inner_rules)
    };
    
    let iter = rule.get_iterator(
        dt_from_ymd_hms(2001, 1, 1, 0, 0, 0), // start
        dt_from_ymd_hms(2019, 1, 30, 0, 0, 0), // from
        dt_from_ymd_hms(2019, 2, 22, 17, 17, 0)); // to

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

#[test]
fn multi_rules() {
    let before_lunch = Rule {
        offset: OffsetKind::Duration(Duration::hours(9)),
        repetition: None,
        length: Duration::hours(4),
        inner_rules: None
    };

    let after_lunch = Rule {
        offset: OffsetKind::Duration(Duration::hours(14)),
        repetition: None,
        length: Duration::hours(4),
        inner_rules: None
    };

    // the work day starts at 9:00am with duration of 9 hours (ends at 6:00pm) 
    // with lunch break from 1:00pm to 2:00pm 
    let eight_hour_day_rule = Rule {
        offset: OffsetKind::Duration(Duration::hours(0)),
        repetition: Some(RepetitionKind::Duration(Duration::days(1))),
        length: Duration::days(1),
        inner_rules: Some(vec![before_lunch, after_lunch])
    };

    // the rule starts at Monday 2001-1-1 at 12:00am, it repeats each 7 days, 
    // so it represents week rule starting from Monday with 5 working days.
    let rule = Rule {
        offset: OffsetKind::DateTime(dt_from_ymd_hms(2001, 1, 1, 0, 0, 0)),
        repetition: Some(RepetitionKind::Duration(Duration::days(7))),
        length: Duration::days(5),
        inner_rules: Some(vec![eight_hour_day_rule])
    };

    let iter = rule.get_iterator(
        dt_from_ymd_hms(2001, 1, 1, 0, 0, 0), // start
        dt_from_ymd_hms(2019, 10, 25, 0, 0, 0), // from // Fri
        dt_from_ymd_hms(2019, 10, 28, 16, 15, 0)); // to

    let intervals: Vec<TimeInterval> = iter.take(4).collect();

    let vec = vec![
            TimeInterval {
                start: dt_from_ymd_hms(2019, 10, 25, 9, 0, 0), 
                end: dt_from_ymd_hms(2019, 10, 25, 13, 00, 0)
            },
            TimeInterval {
                start: dt_from_ymd_hms(2019, 10, 25, 14, 0, 0), 
                end: dt_from_ymd_hms(2019, 10, 25, 18, 00, 0)
            },
            TimeInterval {
                start: dt_from_ymd_hms(2019, 10, 28, 9, 0, 0), 
                end: dt_from_ymd_hms(2019, 10, 28, 13, 00, 0)
            },
            TimeInterval {
                start: dt_from_ymd_hms(2019, 10, 28, 14, 0, 0), 
                end: dt_from_ymd_hms(2019, 10, 28, 16, 15, 0)
            },
            
        ];

    for i in 0..4 {
        assert_eq!(intervals[i].start, vec[i].start, "{}", i);
        assert_eq!(intervals[i].end, vec[i].end, "{}", i);
    }
}