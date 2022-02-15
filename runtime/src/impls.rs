
use crate::{AccountId, Authorship, Balances};
use frame_support::traits::{Currency, Imbalance, OnUnbalanced};

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;


pub struct MockTreasury<F>(sp_std::marker::PhantomData<F>);
impl OnUnbalanced<NegativeImbalanceOf> for MockTreasury<F> {
    fn on_nonzero_unbalanced(amount: NegativeImbalanceOf) {
        // add balance to mock treasury account
        Balances::resolve_creating(&MOCK_TREASURY, amount);
    }
}

pub struct DealWithImbalace;
impl OnUnbalanced<NegativeImbalance> for DealWithImbalace {
    fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
        if let Some(fees) = fees_then_tips.next() {
            // for fees, 80% to treasury, 20% to author
            let mut split = fees.ration(80, 20);
            if let Some(tips) = fees_then_tips.next() {
                // for tips, if any, 80% to treasury, 20% to author (though this can be anything)
                tips.ration_merge_into(80, 20, &mut split);
            }
            MockTreasury::on_unbalanced(split.0);
        }
    }
}