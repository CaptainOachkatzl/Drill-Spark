use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::Resources;

use super::Transaction;

#[derive(Component, Resource)]
pub struct ResourceStore {
  containers: BTreeMap<Resources, usize>,
}

impl ResourceStore {
  pub fn new() -> Self {
    ResourceStore {
      containers: BTreeMap::new(),
    }
  }

  pub fn get_count(&self, resource_type: Resources) -> usize {
    return match self.containers.get(&resource_type) {
      Some(count) => *count,
      None => 0,
    };
  }

  pub fn set_resources(&mut self, resources: BTreeMap<Resources, usize>) {
    self.containers = resources;
  }

  pub fn add_resource(&mut self, transaction: &Transaction) -> bool {
    if !self.verify_not_zero(transaction) {
      return false;
    }

    for cost in transaction.get_costs().iter() {
      if let Some(val) = self.containers.get_mut(&cost.0) {
        *val += cost.1;
      } else {
        self.containers.insert(*cost.0, *cost.1);
      }
    }

    return true;
  }

  pub fn take_resource(&mut self, transaction: &Transaction) -> bool {
    if !self.verify_not_zero(transaction) {
      return false;
    }

    if !self.has_resource(transaction) {
      return false;
    }

    for cost in transaction.get_costs().iter() {
      *self.containers.get_mut(&cost.0).unwrap() -= *cost.1;
    }

    return true;
  }

  pub fn has_resource(&self, transaction: &Transaction) -> bool {
    for cost in transaction.get_costs().iter() {
      if self.get_count(*cost.0) < *cost.1 {
        return false;
      }
    }

    return true;
  }

  pub fn clone_resources(&self) -> BTreeMap<Resources, usize> {
    self.containers.clone()
  }

  fn verify_not_zero(&self, transaction: &Transaction) -> bool {
    for &cost in transaction.get_costs().values() {
      if cost <= 0 {
        return false;
      }
    }
    return true;
  }
}
