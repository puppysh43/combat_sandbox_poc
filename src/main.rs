fn main() {
    //make a new combat encounter
    let combatants = vec![3, 9, 2, 34, 89];
    let mut combat_encounter = CombatEncounter::new(combatants);
    //first make sure that it can even be iterated through at all
    // for _ in 0..combat_encounter.initiative_order.len() {
    for _ in 0..9 {
        //print out the entity id of the currently active entity
        println!(
            "Current entity ID: {}",
            combat_encounter.next_entity().unwrap()
        );
        //then iterate through the encounter
        combat_encounter.complete_turn();
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CombatEncounter {
    ///this vec contains the entity IDs of the entities
    ///in the combat encounter as well as a flag for if they've
    ///gone that round or not. Initiative order is represented
    ///by the order of the vec
    initiative_order: Vec<(i32, bool)>,
    ///the total number of rounds in the combat
    num_rounds: i32,
}

impl CombatEncounter {
    ///creates a new combat encounter given a list of entity IDs arranged in initiative order
    pub fn new(combatants: Vec<i32>) -> Self {
        let mut initiative_order: Vec<(i32, bool)> = Vec::new();
        for entity in combatants.iter() {
            initiative_order.push((*entity, false));
        }
        Self {
            initiative_order,
            num_rounds: 0,
        }
    }
    ///returns the entity ID of the next character in the initiative order
    pub fn next_entity(&self) -> Option<i32> {
        //iterate through all entities and return the first one to not be completed
        for (entity, completed_turn) in self.initiative_order.iter() {
            if !completed_turn {
                return Some(*entity);
            }
        }
        return None;
    }

    ///used to mark a character's turn in the initiative order as complete.
    pub fn complete_turn(&mut self) {
        //iterate through the initiative order
        for (_entity, completed_turn) in self.initiative_order.iter_mut() {
            //when you find the first one that hasn't been completed then mark it as complete
            if !*completed_turn {
                *completed_turn = true;
                //use break to only mark one as complete
                break;
            }
        }
        //finally run a function to see if everyone has gone and if so complete the round
        self.reset_initiative();
    }
    ///used to complete a round, incrementing the num of rounds and resetting the initiative order of every entity
    pub fn reset_initiative(&mut self) {
        //flag for round completion is automatically set to true
        let mut is_complete = true;
        //iterate through the initiative order to check if everyone has completed their turn for this round
        for (_, completed_turn) in self.initiative_order.iter() {
            if !completed_turn {
                is_complete = false;
            }
        }
        //if everyone in the iniative order has taken their turn then increment the round count and reset the initiative order
        //back to the beginning where no one has gone yet.
        if is_complete {
            self.num_rounds += 1;
            for (_entity, completed_turn) in self.initiative_order.iter_mut() {
                *completed_turn = false;
            }
        }
    }
}
