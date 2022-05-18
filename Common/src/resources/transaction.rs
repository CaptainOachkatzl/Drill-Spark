use std::collections::HashMap;

use crate::Resources;

#[derive(Clone, PartialEq, Eq)]
pub struct Transaction {
  resource_costs: HashMap<Resources, usize>,
}

impl Transaction {
  pub fn new() -> Self {
    Transaction { resource_costs: HashMap::new() }
  }

  pub fn new_single(resource_type: Resources, count: usize) -> Self {
    let mut transaction = Transaction::new();
    transaction.add_cost(resource_type, count);
    return transaction;
  }

  pub fn add_cost(&mut self, resource_type: Resources, count: usize) {
    self.resource_costs.insert(resource_type, count);
  }

  pub fn get_costs(&self) -> &HashMap<Resources, usize> {
    &self.resource_costs
  }
}
