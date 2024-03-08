use vars::demo_structs::{demo_firm, demo_balance_sheet};
use vars::algo::valuation::financial_statement::balance_sheet;


#[test]
fn balance_sheet_test(){
    let firm = demo_firm();
    let bs = balance_sheet(firm);
    let demo_bs = demo_balance_sheet();
    assert_eq!(demo_bs, bs)
}