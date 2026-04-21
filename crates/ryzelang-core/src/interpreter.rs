use crate::{RuneValue, Runtime};
use anyhow::{anyhow, Result};
use num_bigint::BigInt;
use num_traits::{One, Zero, ToPrimitive};
use std::time::Duration;
use tree_sitter::{Node, Parser};

pub struct Interpreter {
    pub runtime: Runtime,
    source: String,
}

impl Interpreter {
    pub fn new(source: String) -> Self {
        Self {
            runtime: Runtime::new(),
            source,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_ryze::language())?;
        
        let tree = parser.parse(&self.source, None)
            .ok_or_else(|| anyhow!("Failed to parse source"))?;

        self.eval_node(tree.root_node())
    }

    fn eval_node(&mut self, node: Node<'_>) -> Result<()> {
        let res = match node.kind() {
            "program" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.eval_node(child)?;
                }
                Ok(())
            }
            "block_push" => {
                let block_text = node.utf8_text(self.source.as_bytes())?.to_string();
                self.runtime.push_scroll(block_text);
                Ok(())
            }
            "combo_op" => {
                let name_node = node.child_by_field_name("name")
                    .ok_or_else(|| anyhow!("combo_op missing name"))?;
                let name = name_node.utf8_text(self.source.as_bytes())?.to_string();
                
                if node.child_by_field_name("store").is_some() {
                    // Prevent overwriting built-in primitives
                    if is_reserved(&name) {
                        return Err(anyhow!("Reserved Spell Error: Cannot overwrite built-in spell '{}'.", name));
                    }
                    
                    let block = self.runtime.pop_scroll()
                        .ok_or_else(|| anyhow!("Stack Underflow: R operator requires a block from the Scroll Stack"))?;
                    self.runtime.scroll.insert(name, block);
                    Ok(())
                } else {
                    self.execute_combo(&name)
                }
            }
            "comment" | "[" | "]" => Ok(()),
            _ => {
                if node.is_error() {
                    Err(anyhow!("Syntax Error: Malformed structure or invalid symbol."))
                } else if node.is_missing() {
                    Err(anyhow!("Syntax Error: Incomplete structure."))
                } else {
                    Ok(())
                }
            }
        };

        res.map_err(|e| {
            let start = node.start_position();
            let snippet = node.utf8_text(self.source.as_bytes()).unwrap_or("");
            anyhow!("Runtime Error at Line {}, Col {}: {}\nSource: '{}'", 
                start.row + 1, start.column + 1, e, snippet)
        })
    }

    fn execute_combo(&mut self, combo: &str) -> Result<()> {
        match combo {
            // Stack Manipulation
            "Q" => self.runtime.push_rune(RuneValue::Integer(BigInt::one())),
            "W" => { self.runtime.pop_rune(); },
            "E" => {
                if let Some(val) = self.runtime.rune_stack.last().cloned() {
                    self.runtime.push_rune(val);
                }
            }
            "EW" => {
                let len = self.runtime.rune_stack.len();
                if len >= 2 {
                    self.runtime.rune_stack.swap(len - 1, len - 2);
                }
            }

            // Math
            "QQ" => self.math_op(|a, b| a + b)?,
            "QW" => self.math_op(|a, b| b - a)?,
            "WE" => self.math_op(|a, b| a * b)?,
            "WQ" => self.math_op(|a, b| b / a)?,

            // Logic & Control
            "EQ" => {
                let val = self.runtime.pop_rune()
                    .ok_or_else(|| anyhow!("Stack Underflow: EQ requires 1 Rune"))?;
                match val {
                    RuneValue::Integer(i) => {
                        let res = if i.is_zero() { BigInt::one() } else { BigInt::zero() };
                        self.runtime.push_rune(RuneValue::Integer(res));
                    }
                    RuneValue::FluxCharged(i) => {
                        self.runtime.push_rune(RuneValue::Integer(i));
                        self.runtime.push_rune(RuneValue::Integer(BigInt::one()));
                    }
                    RuneValue::FluxEmpty => {
                        self.runtime.push_rune(RuneValue::Integer(BigInt::zero()));
                    }
                }
            }
            "WEQ" => {
                let cond = self.pop_int()?;
                let false_block = self.runtime.pop_scroll()
                    .ok_or_else(|| anyhow!("Stack Underflow: WEQ requires 2 Scroll blocks"))?;
                let true_block = self.runtime.pop_scroll()
                    .ok_or_else(|| anyhow!("Stack Underflow: WEQ requires 2 Scroll blocks"))?;
                
                if cond > BigInt::zero() {
                    self.execute_block(&true_block)?;
                } else {
                    self.execute_block(&false_block)?;
                }
            }
            "WWW" => {
                let ms = self.pop_int()?;
                let ms_u64 = ms.to_u64().ok_or_else(|| anyhow!("Invalid sleep duration"))?;
                std::thread::sleep(Duration::from_millis(ms_u64));
            }
            "EE" => {
                let val = self.pop_int()?;
                if val.is_zero() {
                    self.runtime.push_rune(RuneValue::FluxEmpty);
                } else {
                    self.runtime.push_rune(RuneValue::FluxCharged(val));
                }
            }

            // I/O
            "EWQ" => {
                let val = self.pop_int()?;
                print!("{}", val);
                use std::io::Write;
                std::io::stdout().flush()?;
            }
            "EQE" => {
                let val = self.pop_int()?;
                let c = u32::try_from(&val).ok()
                    .and_then(std::char::from_u32)
                    .ok_or_else(|| anyhow!("Invalid ASCII character: {}", val))?;
                print!("{}", c);
                use std::io::Write;
                std::io::stdout().flush()?;
            }
            "EEW" => {
                use std::io::Read;
                let mut buf = [0u8; 1];
                std::io::stdin().read_exact(&mut buf)?;
                self.runtime.push_rune(RuneValue::Integer(BigInt::from(buf[0])));
            }
            
            _ => {
                // Custom Combo
                if let Some(block_source) = self.runtime.scroll.get(combo).cloned() {
                    self.execute_block(&block_source)?;
                } else {
                    return Err(anyhow!("Undefined Spell or Combo: '{}'", combo));
                }
            }
        }
        Ok(())
    }

    fn execute_block(&mut self, source: &str) -> Result<()> {
        let inner = source.trim();
        let content = if inner.starts_with('[') && inner.ends_with(']') {
            &inner[1..inner.len()-1]
        } else {
            inner
        };

        let mut sub_interpreter = Interpreter::new(content.to_string());
        sub_interpreter.runtime = std::mem::take(&mut self.runtime);
        sub_interpreter.run()?;
        self.runtime = sub_interpreter.runtime;
        Ok(())
    }

    fn math_op<F>(&mut self, op: F) -> Result<()>
    where F: FnOnce(BigInt, BigInt) -> BigInt 
    {
        let a = self.pop_int().map_err(|e| anyhow!("Math Error (operand 2): {}", e))?;
        let b = self.pop_int().map_err(|e| anyhow!("Math Error (operand 1): {}", e))?;
        self.runtime.push_rune(RuneValue::Integer(op(a, b)));
        Ok(())
    }

    fn pop_int(&mut self) -> Result<BigInt> {
        let val = self.runtime.pop_rune();
        // println!("Popping Rune: {:?}", val);
        match val {
            Some(RuneValue::Integer(i)) => Ok(i),
            _ => Err(anyhow!("Expected integer on Rune Stack, got {:?}", val)),
        }
    }
}

/// Checks if a combo name is a reserved built-in primitive.
fn is_reserved(name: &str) -> bool {
    matches!(
        name,
        "Q" | "W" | "E" | "EW" | 
        "QQ" | "QW" | "WE" | "WQ" | 
        "EQ" | "WEQ" | "WWW" | "EE" | 
        "EWQ" | "EQE" | "EEW"
    )
}
