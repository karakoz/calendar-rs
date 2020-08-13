mod helper;
use helper::dt_from_ymd_hms;

use calendar::rule::Rule;
use calendar::rule_item::RuleItem;
use calendar::offset_kind::OffsetKind;
use calendar::repetition_kind::RepetitionKind;
use calendar::time_interval::TimeInterval;
use chrono::{Duration};

mod iter_tests;

#[test]
fn rule_works() {
    // the work day starts at 9:15am with duration 8h 45min (ends at 6:00pm) 
    let eight_hour_day_rule_item = RuleItem {
        offset: OffsetKind::Duration(Duration::hours(9) + Duration::minutes(15)),
        repetition: Some(RepetitionKind::Duration(Duration::days(1))),
        length: Duration::hours(8) + Duration::minutes(45),
    };

    let eight_hour_day_rule = Rule {
        rule_item: eight_hour_day_rule_item,
        priority: 1,
        value: Some(1),
        inner_rules: None
    };

    let inner_rules = vec![eight_hour_day_rule; 1];

    // the rule starts at Monday 2001-1-1 at 12:00am, it repeats each 7 days, 
    // so it represents week rule starting from Monday with 5 working days.
    let rule_item = RuleItem {
        offset: OffsetKind::DateTime(dt_from_ymd_hms(2001, 1, 1, 0, 0, 0)),
        repetition: Some(RepetitionKind::Duration(Duration::days(7))),
        length: Duration::days(5),
    };

    let rule = Rule {
        rule_item,
        priority: 1,
        value: None,
        inner_rules: Some(inner_rules)
    };
    
    let iter = rule.get_iterator(
        dt_from_ymd_hms(2001, 1, 1, 0, 0, 0), // start
        dt_from_ymd_hms(2019, 1, 30, 0, 0, 0), // from
        dt_from_ymd_hms(2019, 2, 22, 17, 17, 0), // to
    );

    let intervals: Vec<TimeInterval> = iter.take(30).map(|x| x.0).collect();

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
        dt_from_ymd_hms(2019, 1, 30, 11, 11, 0))
        .take(1).map(|x| x.0).collect();

    assert_eq!(little_interval[0].start, dt_from_ymd_hms(2019, 1, 30, 10, 10, 0));
    assert_eq!(little_interval[0].end, dt_from_ymd_hms(2019, 1, 30, 11, 11, 0));

}

// #[test]
// fn multi_rules() {
//     let before_lunch = Rule {
//         offset: OffsetKind::Duration(Duration::hours(9)),
//         repetition: None,
//         length: Duration::hours(4),
//     };

//     let before_lunch = VaRul::from_rule(before_lunch, 1, 1, None);

//     let after_lunch = Rule {
//         offset: OffsetKind::Duration(Duration::hours(14)),
//         repetition: None,
//         length: Duration::hours(4),
//     };

//     let after_lunch = VaRul::from_rule(after_lunch, 1, 1, None);

//     // the work day starts at 9:00am with duration of 9 hours (ends at 6:00pm) 
//     // with lunch break from 1:00pm to 2:00pm 
//     let eight_hour_day_rule = Rule {
//         offset: OffsetKind::Duration(Duration::hours(0)),
//         repetition: Some(RepetitionKind::Duration(Duration::days(1))),
//         length: Duration::days(1),
//     };

//     let eight_hour_day_rule = VaRul::from_rule(eight_hour_day_rule, 1, 1, 
//         Some(vec![before_lunch, after_lunch]));

//     // the rule starts at Monday 2001-1-1 at 12:00am, it repeats each 7 days, 
//     // so it represents week rule starting from Monday with 5 working days.
//     let rule = Rule {
//         offset: OffsetKind::DateTime(dt_from_ymd_hms(2001, 1, 1, 0, 0, 0)),
//         repetition: Some(RepetitionKind::Duration(Duration::days(7))),
//         length: Duration::days(5),
//     };

//     let rule = VaRul::from_rule(rule, 1, 1, Some(vec![eight_hour_day_rule]));

//     let iter = rule.get_iterator(
//         dt_from_ymd_hms(2001, 1, 1, 0, 0, 0), // start
//         dt_from_ymd_hms(2019, 10, 25, 0, 0, 0), // from // Fri
//         dt_from_ymd_hms(2019, 10, 28, 16, 15, 0), // to
//         Replace {});

//     let intervals: Vec<TimeInterval> = iter.take(4).collect();

//     let vec = vec![
//             TimeInterval {
//                 start: dt_from_ymd_hms(2019, 10, 25, 9, 0, 0), 
//                 end: dt_from_ymd_hms(2019, 10, 25, 13, 00, 0)
//             },
//             TimeInterval {
//                 start: dt_from_ymd_hms(2019, 10, 25, 14, 0, 0), 
//                 end: dt_from_ymd_hms(2019, 10, 25, 18, 00, 0)
//             },
//             TimeInterval {
//                 start: dt_from_ymd_hms(2019, 10, 28, 9, 0, 0), 
//                 end: dt_from_ymd_hms(2019, 10, 28, 13, 00, 0)
//             },
//             TimeInterval {
//                 start: dt_from_ymd_hms(2019, 10, 28, 14, 0, 0), 
//                 end: dt_from_ymd_hms(2019, 10, 28, 16, 15, 0)
//             },
            
//         ];

//     for i in 0..4 {
//         assert_eq!(intervals[i].start, vec[i].start, "{}", i);
//         assert_eq!(intervals[i].end, vec[i].end, "{}", i);
//     }
// }

// #[test]
// fn year_repetition_two_days_after_two_twelve_hours() {
//     // the work day starts at 10:00am with duration 12 hours (ends at 10:00pm) 
//     let twelve_hour_day_rule = Rule {
//         offset: OffsetKind::Duration(Duration::hours(10)),
//         repetition: Some(RepetitionKind::Duration(Duration::days(1))),
//         length: Duration::hours(12),
//     };

//     let twelve_hour_day_rule = VaRul::from_rule(twelve_hour_day_rule, 1, 1, None);

//     let inner_rule = Rule {
//         offset: OffsetKind::Duration(Duration::days(0)),
//         repetition: Some(RepetitionKind::Duration(Duration::days(4))),
//         length: Duration::days(2),
//     };
    
//     let inner_rules = vec![VaRul::from_rule(inner_rule, 1, 1, Some(vec![twelve_hour_day_rule]))];

//     let rule = Rule {
//         offset: OffsetKind::DateTime(dt_from_ymd_hms(2001, 1, 1, 0, 0, 0)),
//         repetition: Some(RepetitionKind::Years(1)),
//         length: Duration::days(366),
//     };

//     let rule = VaRul::from_rule(rule, 1, 1, Some(inner_rules));
    
//     let iter = rule.get_iterator(
//         dt_from_ymd_hms(2001, 1, 1, 0, 0, 0), // start
//         dt_from_ymd_hms(2019, 10, 25, 0, 0, 0), // from // Fri
//         dt_from_ymd_hms(2019, 11, 1, 16, 15, 0), // to
//         Replace {});
//     let intervals: Vec<TimeInterval> = iter.take(4).collect();

//     let vec = vec![
//             TimeInterval {
//                 start: dt_from_ymd_hms(2019, 10, 25, 10, 0, 0), 
//                 end: dt_from_ymd_hms(2019, 10, 25, 22, 00, 0)
//             },
//             TimeInterval {
//                 start: dt_from_ymd_hms(2019, 10, 28, 10, 0, 0), 
//                 end: dt_from_ymd_hms(2019, 10, 28, 22, 00, 0)
//             },
//             TimeInterval {
//                 start: dt_from_ymd_hms(2019, 10, 29, 10, 0, 0), 
//                 end: dt_from_ymd_hms(2019, 10, 29, 22, 00, 0)
//             },
//             TimeInterval {
//                 start: dt_from_ymd_hms(2019, 11, 1, 10, 0, 0), 
//                 end: dt_from_ymd_hms(2019, 11, 1, 16, 15, 0)
//             },
            
//         ];

//     for i in 0..4 {
//         assert_eq!(intervals[i].start, vec[i].start, "{}", i);
//         assert_eq!(intervals[i].end, vec[i].end, "{}", i);
//     }
// }