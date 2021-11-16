pub type Priority = [Process; 512];

pub struct VirtualMemoryTable {}

struct Process {
    id: u64,
    mem_table: *mut VirtualMemoryTable, // Pointer to a memory table
}

pub struct Scheduler {
    pub high_priority: Priority,   //150
    pub medium_priority: Priority, //100
    pub low_priority: Priority,    // 50
    pub next_pid: u64,
}

impl Scheduler {
    pub fn bump_up() {}
    pub fn bump_down() {}
    pub fn schedule(&mut self) {
        let current_pid = self.next_pid;
        self.next_pid += 1;
    }
}
