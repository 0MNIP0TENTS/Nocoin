// src/state_channels.rs

pub struct StateChannel {
    pub participants: Vec<String>,
    pub balances: Vec<u64>, // Balances corresponding to each participant
    pub is_active: bool,
}

impl StateChannel {
    pub fn new(participants: Vec<String>, balances: Vec<u64>) -> Self {
        assert_eq!(participants.len(), balances.len(), "Participants and balances must match in length.");
        Self {
            participants,
            balances,
            is_active: true,
        }
    }

    pub fn update_balances(&mut self, sender_idx: usize, receiver_idx: usize, amount: u64) -> bool {
        if self.is_active && self.balances[sender_idx] >= amount {
            self.balances[sender_idx] -= amount;
            self.balances[receiver_idx] += amount;
            true
        } else {
            false
        }
    }

    pub fn close_channel(&mut self) {
        self.is_active = false;
        // Logic to finalize balances on the main chain
        println!("State channel closed. Final balances: {:?}", self.balances);
    }
}
