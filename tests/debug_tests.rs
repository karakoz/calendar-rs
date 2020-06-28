use calendar::{
    rule_item::RuleItem,
    offset_kind::OffsetKind,
    repetition_kind::RepetitionKind
};
use chrono::{NaiveDate, Duration};


#[test]
fn rule_item_ser() {
    let rule = RuleItem {
        offset: OffsetKind::DateTime(NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11)),
        repetition: None,
        length: Duration::seconds(3600)
    };

    let serialized = serde_json::to_string(&rule).unwrap();

    assert_eq!("{\"offset\":{\"DateTime\":\"2016-07-08T09:10:11+00:00\"},\"repetition\":null,\"length\":\"3600\"}", serialized);

}


#[test]
fn rule_item_deser() {
    let deser: RuleItem = serde_json::from_str("{\"offset\":{\"DateTime\":\"2020-02-14T01:21:00.000Z\"},\"repetition\":{\"Duration\":222},\"length\":11}").unwrap();

    let ar = RuleItem {
        offset: OffsetKind::DateTime(NaiveDate::from_ymd(2020, 2, 14).and_hms(1, 21, 0)),
        repetition: Some(RepetitionKind::Duration(Duration::seconds(222))),
        length: Duration::seconds(11)
    };
    assert_eq!(ar, deser);
}