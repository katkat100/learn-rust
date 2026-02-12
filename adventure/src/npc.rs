use colored::Colorize;

use crate::items;
use crate::items::Item;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NPCType {
    Cleaner,
    Merchant,
    QuestGiver {
        quest: String,
        quest_taken: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogueState {
    Initial,
    Introduced,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NPC {
    pub name: String,
    description: String,
    pub npc_type: NPCType,
    pub dialogue_state: DialogueState,
    pub items: Vec<Item>,
}

impl NPC {
    pub fn new(name: String, description: String, npc_type: NPCType, items: Vec<Item>) -> Self {
        NPC {
            name,
            description,
            npc_type,
            dialogue_state: DialogueState::Initial,
            items,
        }
    }

    // talk with npc
    pub fn talk(&mut self) -> String {
        match self.npc_type {
            NPCType::Cleaner => {
                match &self.dialogue_state {
                    DialogueState::Initial => {
                        let mut dialogue = String::new();
                        dialogue.push_str(&format!("???: \"The name's {}. I clean up after the adventurers.\"\n", self.name.green()));
                        dialogue.push_str(&format!("{}: \"Bring me any junk you find — cups, spoons, whatever.\"\n", self.name.green()));
                        dialogue.push_str(&format!("{}: \"I'll give you 1 gold for each piece.\"\n", self.name.green()));
                        self.dialogue_state = DialogueState::Introduced;
                        dialogue
                    }
                    DialogueState::Introduced => {
                        let mut dialogue = String::new();
                        dialogue.push_str(&format!("{}: \"Got any junk for me?\"", self.name.green()));
                        dialogue
                    }
                }
            }
            NPCType::Merchant => {
                match &self.dialogue_state {
                    DialogueState::Initial => {
                        let mut dialogue = String::new();
                        dialogue.push_str(&format!("???: \"Hey there! Name's {}, I sell and buy all kinds of stuff, but I'm running quite low on things at the moment.\"\n", self.name.green()));
                        dialogue.push_str(&format!("{}: \"What can I help you with?\"\n", self.name.green()));
                        self.dialogue_state = DialogueState::Introduced;
                        dialogue
                    }
                    DialogueState::Introduced => {
                        let mut dialogue = String::new();
                        dialogue.push_str(&format!("{}: \"What can I help you with?\"", self.name.green()));
                        dialogue
                    }
                }
            }
            NPCType::QuestGiver { quest: _, quest_taken: bool } => {
                match &self.dialogue_state {
                    DialogueState::Initial => {
                        let mut dialogue = String::new();
                        dialogue.push_str(&format!("???: \"Oh goodness! I need some help if you're willing to lend a hand.\"\n"));
                        dialogue.push_str(&format!("???: \"My name is {}, I've been looking for my grandmothers ring that was stolen from me!\"\n", self.name.green()));
                        dialogue.push_str(&format!("{}: \"If you find the ring, bring it back to me.\"\n", self.name.green()));
                        self.dialogue_state = DialogueState::Introduced;
                        dialogue
                    }
                    DialogueState::Introduced => {
                        let mut dialogue = String::new();
                        dialogue.push_str(&format!("{}: \"Were you able to find the ring?\"", self.name.green()));
                        dialogue
                    }
                }
            }
        }
    }

    pub fn available_actions(&self) -> Vec<&str> {
        match(&self.npc_type, &self.dialogue_state) {
            (NPCType::Cleaner, DialogueState::Introduced) => vec!["Trade junk", "Leave"],
            (NPCType::Merchant, DialogueState::Introduced) => vec!["Buy", "Sell", "Leave"],
            (NPCType::QuestGiver {quest: _, quest_taken: _}, DialogueState::Introduced) => vec!["Deliver Item", "Leave"],
            (_, DialogueState::Initial) => vec!["Leave"],
        }
    }
}

pub fn create_npc_cleaner() -> NPC {
    NPC::new("Grim".to_string(), "Grim grabs grimy gunk from the grotto after every grueling grand adventure.".to_string(), NPCType::Cleaner, vec![])
}
pub fn create_npc_merchant() -> NPC {
    NPC::new("Dolly".to_string(), "Dolly deals deap down in the dungeon's dark".to_string(), NPCType::Merchant, vec![items::create_small_health_potion()])
}
pub fn create_npc_regular_guy() -> NPC {
    NPC::new("Jacob".to_string(), "Jacob is a regular guy".to_string(), NPCType::QuestGiver {quest: "Ring".to_string(), quest_taken: false}, vec![])
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_npc() {
        let npc = NPC::new(
            "John".to_string(),
            "A friendly cleaner".to_string(),
            NPCType::Cleaner,
            vec![],
        );
        assert_eq!(npc.name, "John");
        assert_eq!(npc.description, "A friendly cleaner");
        assert_eq!(npc.npc_type, NPCType::Cleaner);
    }
}
