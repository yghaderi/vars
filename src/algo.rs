pub mod valuation {
    pub mod initialize {
        fn firm() {}
    }

    pub mod financial_statement {
        use crate::models::{BalanceSheet, Firm};

        pub fn balance_sheet(firm: Firm) -> BalanceSheet {
            let mut fixed_asset: u64 = 0;
            let mut inventory: u64 = 0;
            for cc in firm.cost_centers {
                for input in cc.input {
                    if input.fixed_assets.is_some() {
                        for fa in input.fixed_assets.unwrap() {
                            fixed_asset += fa.calc_book_value()
                        }
                    }
                }
            }
            BalanceSheet {
                fixed_asset,
                inventory,
            }
        }

        fn income_statement() {}

        fn cash_flow() {}

        fn free_cash_flow() {}
    }
}
