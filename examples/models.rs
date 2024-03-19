use chrono::NaiveDate;

use vars::models::{RateChange, BaseRateChange, FinancialYear, Firm, FirmCategory, BaseParam};

fn main() {
    let financial_year = FinancialYear {
        date: NaiveDate::from_ymd_opt(2023, 10, 10).unwrap(),
        length: 5,
    };
    let base_rate_change = BaseRateChange{
        id:"1023".to_string(),
        name:"دلار".to_string(),
        rates:vec![
            RateChange{date: NaiveDate::from_ymd_opt(2023, 12, 29).unwrap(), f: 0.25},
            RateChange{date: NaiveDate::from_ymd_opt(2024, 12, 29).unwrap(), f: 0.25},
            RateChange{date: NaiveDate::from_ymd_opt(2025, 12, 29).unwrap(), f: 0.2},
            RateChange{date: NaiveDate::from_ymd_opt(2026, 12, 29).unwrap(), f: 0.2},
        ]
    };

    let firm = Firm {
        id: String::from("1"),
        name: String::from("شپدیس"),
        base_param : BaseParam{
            financial_year,
            base_rate_change
        },
        category: FirmCategory::Production,
        cost_centers: vec![],
    };
}
