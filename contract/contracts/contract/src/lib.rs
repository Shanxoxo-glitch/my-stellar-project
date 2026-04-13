#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Map, symbol_short
};

#[contracttype]
#[derive(Clone)]
pub struct Task {
    pub id: u64,
    pub creator: Address,
    pub description: String,
    pub reward: i128,
    pub completed: bool,
    pub worker: Option<Address>,
}

#[contract]
pub struct TaskMarketplace;

#[contractimpl]
impl TaskMarketplace {

    // Storage keys
    fn task_count(e: &Env) -> u64 {
        e.storage().instance().get(&symbol_short!("COUNT")).unwrap_or(0)
    }

    fn set_task_count(e: &Env, count: u64) {
        e.storage().instance().set(&symbol_short!("COUNT"), &count);
    }

    fn tasks(e: &Env) -> Map<u64, Task> {
        e.storage().instance().get(&symbol_short!("TASKS")).unwrap_or(Map::new(e))
    }

    fn set_tasks(e: &Env, tasks: Map<u64, Task>) {
        e.storage().instance().set(&symbol_short!("TASKS"), &tasks);
    }

    // Create a new task
    pub fn create_task(e: Env, creator: Address, description: String, reward: i128) -> u64 {
        creator.require_auth();

        let mut tasks = Self::tasks(&e);
        let mut count = Self::task_count(&e);

        count += 1;

        let task = Task {
            id: count,
            creator: creator.clone(),
            description,
            reward,
            completed: false,
            worker: None,
        };

        tasks.set(count, task);

        Self::set_tasks(&e, tasks);
        Self::set_task_count(&e, count);

        count
    }

    // Accept a task
    pub fn accept_task(e: Env, worker: Address, task_id: u64) {
        worker.require_auth();

        let mut tasks = Self::tasks(&e);

        let mut task = match tasks.get(task_id) {
            Some(t) => t,
            None => panic!("Task does not exist"),
        };

        if task.completed {
            panic!("Task already completed");
        }

        if task.worker.is_some() {
            panic!("Task already taken");
        }

        task.worker = Some(worker);
        tasks.set(task_id, task);

        Self::set_tasks(&e, tasks);
    }

    // Complete a task
    pub fn complete_task(e: Env, creator: Address, task_id: u64) {
        creator.require_auth();

        let mut tasks = Self::tasks(&e);

        let mut task = match tasks.get(task_id) {
            Some(t) => t,
            None => panic!("Task does not exist"),
        };

        if task.creator != creator {
            panic!("Only creator can mark complete");
        }

        if task.completed {
            panic!("Already completed");
        }

        task.completed = true;
        tasks.set(task_id, task);

        Self::set_tasks(&e, tasks);

        // TODO: Add token reward transfer logic here
    }

    // Get a task
    pub fn get_task(e: Env, task_id: u64) -> Task {
        let tasks = Self::tasks(&e);

        match tasks.get(task_id) {
            Some(t) => t,
            None => panic!("Task does not exist"),
        }
    }
}