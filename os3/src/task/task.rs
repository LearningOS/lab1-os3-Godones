//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;
use super::TaskContext;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    pub first_run_time: usize, // the first time the task is run
    pub syscall_times:[u32; MAX_SYSCALL_NUM], // the times of syscall
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}


impl TaskControlBlock {
    /// return the first time the task is run
    pub fn get_first_run_time(&self) -> usize {
        self.first_run_time
    }
    /// Returns the system call and the corresponding number of times
    pub fn get_syscall_times(&self) -> [u32; MAX_SYSCALL_NUM] {
        self.syscall_times
    }
    pub fn update_first_run_time(&mut self, time: usize) {
        self.first_run_time = time;
    }
    pub fn update_syscall_times(&mut self, syscall_id:usize) {
        self.syscall_times[syscall_id] += 1;
    }
}
