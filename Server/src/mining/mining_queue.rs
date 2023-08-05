use std::time::Duration;

use bevy::{prelude::*, time::Timer};
use drillspark_common_lib::{game_component::TileType, Position};
use xs_bevy_core_2d::{path_exists, Grid};

pub type IsMineableParams<'a> = (
    &'a Grid<Entity>,
    Position,
    &'a dyn Fn(Entity) -> Position,
    &'a dyn Fn(Entity) -> TileType,
    &'a dyn Fn(Entity) -> bool,
    &'a dyn Fn(Entity) -> bool,
);

#[derive(Component)]
pub struct MiningQueue {
    queue: Vec<Entity>,
    timer: Timer,
    current_workload: Option<Entity>,
}

impl MiningQueue {
    pub fn new() -> Self {
        MiningQueue {
            queue: Vec::new(),
            timer: Timer::from_seconds(2., TimerMode::Repeating),
            current_workload: None,
        }
    }

    pub fn schedule_workload(&mut self, entity: Entity) {
        self.queue.push(entity);
    }

    pub fn remove_from_schedule(&mut self, entity: Entity) {
        self.pop_workload(entity);
        if Some(entity) == self.current_workload {
            self.current_workload = None;
        }
    }

    pub fn update(&mut self, delta: Duration, mineable_params: IsMineableParams) -> Option<Entity> {
        if !self.is_working() {
            if self.has_work() {
                self.start_next_workload(None, mineable_params)
            }
            return None;
        }

        self.timer.tick(delta);
        if self.timer.finished() {
            return self.finish_current_workload(mineable_params);
        }

        None
    }

    #[allow(dead_code)]
    pub fn get_current_mining_location(&self) -> Option<Entity> {
        self.current_workload
    }

    fn is_working(&self) -> bool {
        self.current_workload.is_some()
    }

    fn has_work(&self) -> bool {
        self.queue.len() > 0
    }

    fn finish_current_workload(&mut self, mineable_params: IsMineableParams) -> Option<Entity> {
        let ret = self.pop_currently_mining();
        self.start_next_workload(ret, mineable_params);
        ret
    }

    fn start_next_workload(&mut self, just_finished: Option<Entity>, mineable_params: IsMineableParams) {
        self.current_workload = self.next_mineable(just_finished, mineable_params);

        if self.is_working() {
            self.timer.reset();
        }
    }

    fn pop_currently_mining(&mut self) -> Option<Entity> {
        let current_entity = self.current_workload?;

        self.pop_workload(current_entity)
    }

    fn pop_workload(&mut self, workload: Entity) -> Option<Entity> {
        let Some((index, _)) = self.queue.iter().enumerate().find(|(_, &entity)| entity == workload) else {
            return None;
        };

        Some(self.queue.remove(index))
    }

    fn next_mineable(&self, just_finished: Option<Entity>, mineable_params: IsMineableParams) -> Option<Entity> {
        let &next = self.queue.iter().find(|&x| self.is_mineable(*x, just_finished, mineable_params))?;
        Some(next)
    }

    fn is_mineable(
        &self,
        entity: Entity,
        just_finished: Option<Entity>,
        (grid, warpgate_position, get_position, get_tile_type, is_revealed, is_owned): IsMineableParams,
    ) -> bool {
        let tile_type = get_tile_type(entity);
        let position = get_position(entity);

        if !tile_type.is_tile_type_minable() || position.0 == warpgate_position.0 {
            return false;
        }

        let is_pathable_tile = |entity| {
            if !is_revealed(entity) || !is_owned(entity) {
                return false;
            }

            if just_finished.is_some() && just_finished.unwrap() == entity {
                return true;
            };

            let position = get_position(entity);
            if position.0 == warpgate_position.0 {
                return true;
            }

            match get_tile_type(grid.get_value_by_position(get_position(entity).0).unwrap()) {
                TileType::Ground | TileType::Building(_) => true,
                _ => false,
            }
        };
        path_exists(grid, get_position(entity).0, warpgate_position.0, &is_pathable_tile)
    }
}
