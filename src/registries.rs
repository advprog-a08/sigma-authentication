use std::collections::HashMap;
use std::sync::Arc;

use crate::strategies::{AuthStrategy, PasswordStrategy, GoogleStrategy};

pub struct StrategyRegistry {
    strategies: HashMap<&'static str, Arc<dyn AuthStrategy>>,
}

impl StrategyRegistry {
    pub fn new() -> Self {
        let mut strategies: HashMap<_, Arc<dyn AuthStrategy>> = HashMap::new();
        strategies.insert("password", Arc::new(PasswordStrategy));
        strategies.insert("google", Arc::new(GoogleStrategy));

        StrategyRegistry { strategies }
    }

    pub fn get(&self, kind: &str) -> Option<Arc<dyn AuthStrategy>> {
        self.strategies.get(kind).cloned()
    }
}

impl Default for StrategyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

