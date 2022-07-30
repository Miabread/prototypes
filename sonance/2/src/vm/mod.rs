mod error;
mod frames;
mod step;

pub use crate::vm::{
    error::{Result, VMError},
    frames::{Frame, Frames},
};
use crate::{device::Device, instruction::Instruction};

#[derive(Debug, Clone, PartialEq)]
pub struct VM<D: Device> {
    pub instructions: Vec<u8>,
    pub instruction_index: u8,
    pub current_instruction: Instruction,
    pub stack: Vec<u8>,
    pub frames: Frames,
    pub device: Option<D>,
}

impl<D: Device> Default for VM<D> {
    fn default() -> Self {
        Self {
            instructions: vec![Instruction::Halt as u8],
            instruction_index: 0,
            current_instruction: Instruction::Halt,
            stack: vec![],
            frames: Default::default(),
            device: None,
        }
    }
}

impl<D: Device> VM<D> {
    pub fn new(instructions: Vec<u8>) -> Self {
        Self {
            instructions,
            ..Default::default()
        }
    }

    pub fn with_device(mut self, device: D) -> Self {
        self.device = Some(device);
        self
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.step()? {}
        Ok(())
    }

    fn device(&mut self) -> Result<&mut D> {
        Ok(self.device.as_mut().unwrap())
    }

    fn pop(&mut self) -> Result<u8> {
        self.stack.pop().ok_or(VMError::EmptyStack(
            self.current_instruction,
            self.instruction_index,
        ))
    }

    fn pop_u32(&mut self) -> Result<u32> {
        // They're stored in le byte order, but because popping is reversing them, we can cheese it by loading as if they were be byte order
        Ok(u32::from_be_bytes([
            self.pop()?,
            self.pop()?,
            self.pop()?,
            self.pop()?,
        ]))
    }

    fn unary_op(&mut self, body: impl FnOnce(u8) -> u8) -> Result<()> {
        let a = self.pop()?;
        self.stack.push(body(a));
        Ok(())
    }

    fn binary_op<F>(&mut self, body: F) -> Result<()>
    where
        F: FnOnce(u8, u8) -> u8,
    {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stack.push(body(a, b));
        Ok(())
    }
}
