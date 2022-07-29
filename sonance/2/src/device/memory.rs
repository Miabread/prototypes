use std::io::{stdin, stdout, Read, Write};

use crate::device::Device;

/// A Device for storing data or performing IO
///
/// - `0`: Perform IO
///     - Read: Result of previous IO
///     - Write:
///         - `0`: Read all stdin
///         - `1`: Write all stdout
///         - `10`: Read any stdin
///         - `11`: Write any stdout
///
/// - `1`: Memory Length
/// - `2`: IO Slice Start
/// - `3`: IO Slice End
/// - `4..9`: Reserved
/// - `10..`: Memory
///
#[derive(Default)]
pub struct Memory {
    pub memory: Vec<u8>,
    perform_io: Option<Box<dyn FnMut(&mut Memory, u8) -> u8>>,
    io_result: u8,
    io_start: u8,
    io_end: u8,
    fill_value: u8,
}

impl Memory {
    const IO_REGISTER: u32 = 0;
    const LEN_REGISTER: u32 = 1;
    const IO_START_REGISTER: u32 = 2;
    const IO_END_REGISTER: u32 = 3;
    const FILL_VALUE_REGISTER: u32 = 4;
    const MEM_START: usize = 10;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_io_mock<F>(perform_io: F) -> Self
    where
        F: FnMut(&mut Memory, u8) -> u8 + 'static,
    {
        Self {
            perform_io: Some(Box::new(perform_io)),
            ..Default::default()
        }
    }

    pub fn io_slice(&self) -> &[u8] {
        &self.memory[self.io_start as usize..self.io_end as usize]
    }

    pub fn io_slice_mut(&mut self) -> &mut [u8] {
        &mut self.memory[self.io_start as usize..self.io_end as usize]
    }

    fn standard_perform_io(&mut self, instruction: u8) -> u8 {
        (match instruction {
            0 => {
                stdin().read_exact(self.io_slice_mut()).unwrap();
                self.memory.len()
            }
            1 => {
                stdout().write_all(self.io_slice()).unwrap();
                self.memory.len()
            }

            10 => stdin().read(self.io_slice_mut()).unwrap(),
            11 => stdout().write(self.io_slice()).unwrap(),

            _ => return self.io_result,
        }) as u8
    }
}

impl Device for Memory {
    fn read(&mut self, index: u32) -> u8 {
        match index {
            Self::IO_REGISTER => self.io_result,
            Self::LEN_REGISTER => self.memory.len() as u8,
            Self::IO_START_REGISTER => self.io_start,
            Self::IO_END_REGISTER => self.io_end,
            Self::FILL_VALUE_REGISTER => self.fill_value,

            _ => self.memory[index as usize - Self::MEM_START],
        }
    }

    fn write(&mut self, index: u32, value: u8) {
        match index {
            Self::IO_REGISTER => {
                if let Some(mut perform_io) = self.perform_io.take() {
                    self.io_result = perform_io(self, value);
                    self.perform_io = Some(perform_io);
                } else {
                    self.io_result = self.standard_perform_io(value);
                }
            }

            Self::LEN_REGISTER => self.memory.resize(value as usize, self.fill_value),
            Self::IO_START_REGISTER => self.io_start = value,
            Self::IO_END_REGISTER => self.io_end = value,
            Self::FILL_VALUE_REGISTER => self.fill_value = value,

            _ => self.memory[index as usize - Self::MEM_START] = value,
        }
    }
}
