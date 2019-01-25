use std::cmp::min;

pub struct PendingUpgrade {
    seconds: i32,
}

impl PendingUpgrade {
    pub fn new(seconds: i32) -> PendingUpgrade {
        PendingUpgrade {
            seconds
        }
    }

    pub fn update_and_check_if_time(&mut self) -> bool {
        self.seconds -= 1;
        if self.seconds == -1 {
            return true;
        }
        else {
            return false;
        }
    }
}

pub struct LarvaeProduction {
    current_larvae: i32,
    max_larvae: i32,
    timers: Vec<i32>,
}

impl LarvaeProduction {
    pub fn new() -> LarvaeProduction {
        let mut timers = Vec::with_capacity(1);
        timers.push(11);
        LarvaeProduction {
            current_larvae: 3,
            max_larvae: 3,
            timers,
        }
    }

    pub fn update(&mut self) {
        if self.get_current_larvae() < self.get_max_larvae() {
            for timer in &mut self.timers {
                *timer -= 1;
                if *timer == -1 {
                    *timer = 11;
                    self.current_larvae += 1;
                }
            }
        }
    }

    pub fn get_current_larvae(&self) -> i32 {
        self.current_larvae
    }

    pub fn get_max_larvae(&self) -> i32 {
        self.max_larvae
    }

    pub fn consume_larva(&mut self) {
        self.current_larvae -= 1;
    }

    pub fn add_new_expansion(&mut self) {
        self.max_larvae += 3;
        self.timers.push(11);
    }
}

pub struct BuildOrderSimulator {
    build_order: crate::BuildOrder,
    next_action_index: usize,

    money: i32,
    income: i32,
    max_workers_mining: i32,
    max_units: i32,

    larvae: LarvaeProduction,
    pending_workers: Vec<PendingUpgrade>,
    pending_expansions: Vec<PendingUpgrade>,
    pending_overlords: Vec<PendingUpgrade>,

    simulation_seconds: i32
}

impl BuildOrderSimulator {
    pub fn new(build_order: crate::BuildOrder, start_index: usize) -> BuildOrderSimulator {
        BuildOrderSimulator{
            build_order,
            next_action_index: start_index,
            money: 50,
            income: 12,
            max_workers_mining: 16,
            max_units: 14,
            simulation_seconds: 0,
            larvae: LarvaeProduction::new(),
            pending_workers: Vec::new(),
            pending_expansions: Vec::new(),
            pending_overlords: Vec::new(),
        }
    }

    fn has_timeouted(&self) -> bool {
        self.simulation_seconds >= 60 * 10
    }

    fn is_worker_build_action(&self, action: &char) -> bool {
        action.to_owned() == 'W'
    }

    fn is_expansion_build_action(&self, action: &char) -> bool {
        action.to_owned() == '#'
    }

    fn is_overlord_build_action(&self, action: &char) -> bool {
        action.to_owned() == 'O'
    }

    fn can_build_worker(&self) -> bool {
        self.money >= 50 && self.larvae.current_larvae > 0
    }

    fn can_build_expansion(&self) -> bool {
        self.money >= 300
    }

    fn can_build_overlord(&self) -> bool {
        self.money >= 100 && self.larvae.current_larvae > 0
    }

    fn build_worker(&mut self) {
        self.money -= 50;
        self.larvae.consume_larva();
        self.next_action_index += 1;
        self.pending_workers.push(PendingUpgrade::new(12));
    }

    fn build_expansion(&mut self) {
        self.money -= 300;
        self.next_action_index += 1;
        self.pending_expansions.push(PendingUpgrade::new(80));
    }

    fn build_overlord(&mut self) {
        self.money -= 100;
        self.larvae.consume_larva();
        self.next_action_index += 1;
        self.pending_overlords.push(PendingUpgrade::new(18));
    }

    pub fn measure_duration(&mut self) -> i32 {
        println!("Income: {}", self.income);
        println!("Max workers mining: {}", self.max_workers_mining);
        println!("Timeouted: {}", self.has_timeouted());
        while min(self.income, self.max_workers_mining) != 32 && !self.has_timeouted() {
            self.money += min(self.income, self.max_workers_mining);
            self.larvae.update();
            for pending in &mut self.pending_workers {
                if pending.update_and_check_if_time() {
                    self.income += 1;
                }
            }
            for pending in &mut self.pending_expansions {
                if pending.update_and_check_if_time() {
                    self.max_workers_mining += 16;
                    self.larvae.add_new_expansion();
                }
            }
            for pending in &mut self.pending_overlords {
                if pending.update_and_check_if_time() {
                    self.max_units += 8;
                }
            }

            if self.next_action_index < self.build_order.get_action_count() {
                let action = self.build_order.get_action(self.next_action_index);
                if self.is_worker_build_action(&action) && self.can_build_worker() {
                    self.build_worker();
                }
                else if self.is_expansion_build_action(&action) && self.can_build_expansion() {
                    self.build_expansion();
                }
                else if self.is_overlord_build_action(&action) && self.can_build_overlord() {
                    self.build_overlord();
                }
            }

            self.simulation_seconds += 1;
        }
        self.simulation_seconds
    }
}
