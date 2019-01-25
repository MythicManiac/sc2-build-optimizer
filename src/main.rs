mod simulation;

//use rayon::prelude::*;

pub struct BuildOrder {
    sequence: String,
}

impl BuildOrder {
    pub fn new(sequence: String) -> BuildOrder {
        return BuildOrder {
            sequence
        };
    }

    pub fn get_action_count(&self) -> usize {
        self.sequence.chars().count()
    }

    pub fn get_action(&self, index: usize) -> char {
        self.sequence.chars().nth(index).unwrap()
    }

    pub fn get_expansions(&self) -> i32 {
        self.sequence.matches('#').count() as i32
    }

    pub fn get_overlords(&self) -> i32 {
        self.sequence.matches('O').count() as i32
    }

    pub fn get_workers(&self) -> i32 {
        self.sequence.matches('W').count() as i32
    }

    pub fn get_max_income(&self) -> i32 {
        self.get_expansions() * 16
    }

    pub fn get_supply(&self) -> i32 {
        self.get_overlords() * 8 + self.get_expansions() * 6
    }

    pub fn get_is_complete(&self) -> bool {
        self.get_workers() == 64 && self.get_max_income() == 64
    }

    pub fn add_next(&self, action: &str) -> BuildOrder {
        BuildOrder::new(self.sequence.clone() + action)
    }

    pub fn get_next_sequences(&self) -> Vec<BuildOrder> {
        // Add IsComplete check
        let mut result = Vec::with_capacity(3);

        if self.get_expansions() < 4 {
            result.push(self.add_next("#"));
        }

        if self.get_workers() < 64 { // Add supply check
            result.push(self.add_next("W"));
        }

        return result;
    }
}


//fn do_stuff(input: &Sequence) -> String {
//    input.sequence.clone() + &input.sequence
//}
//
//fn do_stuff_for_all(input: &[Sequence]) -> Vec<String> {
//    input.par_iter()
//        .map(|i| do_stuff(&i))
//        .collect()
//}


fn main() {
//    let mut input = Vec::with_capacity(4);
//    input.push(Sequence::new("Test1".to_owned()));
//    input.push(Sequence::new("1tseT".to_owned()));
//    input.push(Sequence::new("dond".to_owned()));
//    let result = do_stuff_for_all(&input);
//    for i in 0..3  {
//        println!("{}", result[i]);
//    }
//    let start = BuildOrder::new("#OWW".to_owned());
//    let results = start.get_next_sequences();
//    for entry in &results {
//        println!("{}", entry.sequence);
//    }
//    let mut larvae = simulation::LarvaeProduction::new();
//    larvae.consume_larva();
//    println!("{}", larvae.get_current_larvae());
//    larvae.consume_larva();
//    println!("{}", larvae.get_current_larvae());
//    for _ in 0..12 {
//        larvae.update();
//    }
//    println!("{}", larvae.get_current_larvae());
    let mut simulation = simulation::BuildOrderSimulator::new(
        BuildOrder::new("#WWWWWWWWWWWW#WWWWWWWWWWWWWWWWWWWW".to_owned()),
        12
    );
    println!("Running simulation...");
    println!("{}", simulation.measure_duration());
}
