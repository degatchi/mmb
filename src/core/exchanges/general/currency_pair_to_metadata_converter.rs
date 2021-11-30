#[cfg(test)]
use mockall::automock;

use std::collections::HashMap;
use std::sync::Arc;

use crate::core::exchanges::common::CurrencyPair;
use crate::core::exchanges::common::ExchangeAccountId;
use crate::core::exchanges::general::symbol::Symbol;
use crate::core::exchanges::general::exchange::Exchange;
use crate::core::infrastructure::WithExpect;

#[derive(Clone)]
pub struct CurrencyPairToMetadataConverter {
    exchanges_by_id: HashMap<ExchangeAccountId, Arc<Exchange>>,
}

#[cfg_attr(test, automock)]
impl CurrencyPairToMetadataConverter {
    pub fn new(exchanges_by_id: HashMap<ExchangeAccountId, Arc<Exchange>>) -> Arc<Self> {
        Arc::new(Self { exchanges_by_id })
    }

    pub(crate) fn get_symbol(
        &self,
        exchange_account_id: ExchangeAccountId,
        currency_pair: CurrencyPair,
    ) -> Arc<Symbol> {
        let exchange = self
            .exchanges_by_id
            .get(&exchange_account_id)
            .with_expect(|| {
                format!(
                    "get_symbol failed to get exchange by id: {}",
                    exchange_account_id
                )
            });
        exchange
            .get_symbol(currency_pair)
            .expect("failed to get currency pair")
    }

    pub(crate) fn exchanges_by_id(&self) -> &HashMap<ExchangeAccountId, Arc<Exchange>> {
        &self.exchanges_by_id
    }
}

#[cfg(test)]
crate::impl_mock_initializer!(MockCurrencyPairToMetadataConverter);
