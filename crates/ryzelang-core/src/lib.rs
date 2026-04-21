use num_bigint::BigInt;
use std::collections::HashMap;
use std::fmt;

pub mod interpreter;

#[derive(Clone, Debug)]
pub enum RuneValue {
    Integer(BigInt),
    FluxEmpty,
    FluxCharged(BigInt),
}

impl fmt::Display for RuneValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuneValue::Integer(i) => write!(f, "{}", i),
            RuneValue::FluxEmpty => write!(f, "Flux(Empty)"),
            RuneValue::FluxCharged(i) => write!(f, "Flux(Charged: {})", i),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub source: String,
    pub node_id: usize, // Reference into the AST if we were walking trees, or just the code
}

#[derive(Default, Debug)]
pub struct Runtime {
    pub rune_stack: Vec<RuneValue>,
    pub scroll_stack: Vec<String>, // Simplification: store block source code
    pub scroll: HashMap<String, String>, // The Environment: Name -> Block Source
}

impl Runtime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format_stacks(&self) -> String {
        let mut out = String::new();
        out.push_str("--- Rune Stack ---\n");
        if self.rune_stack.is_empty() {
            out.push_str("  (empty)\n");
        } else {
            for (i, val) in self.rune_stack.iter().enumerate().rev() {
                out.push_str(&format!("  {:02}: {}\n", i, val));
            }
        }
        out.push_str("\n--- Scroll Stack ---\n");
        if self.scroll_stack.is_empty() {
            out.push_str("  (empty)\n");
        } else {
            for (i, val) in self.scroll_stack.iter().enumerate().rev() {
                let mut display = val.replace("\n", " ");
                if display.len() > 50 {
                    display.truncate(47);
                    display.push_str("...");
                }
                out.push_str(&format!("  {:02}: {}\n", i, display));
            }
        }
        out
    }

    pub fn push_rune(&mut self, val: RuneValue) {
        self.rune_stack.push(val);
    }

    pub fn pop_rune(&mut self) -> Option<RuneValue> {
        self.rune_stack.pop()
    }

    pub fn push_scroll(&mut self, block: String) {
        self.scroll_stack.push(block);
    }

    pub fn pop_scroll(&mut self) -> Option<String> {
        self.scroll_stack.pop()
    }
}
