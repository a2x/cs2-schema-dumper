use crate::error::Result;
use crate::remote::Process;

pub struct SchemaTypeDeclaredClass<'a> {
    process: &'a Process,
    address: usize,
}

impl<'a> SchemaTypeDeclaredClass<'a> {
    pub fn new(process: &'a Process, address: usize) -> Self {
        Self { process, address }
    }

    pub fn name(&self) -> Result<String> {
        self.process
            .read_string(self.process.read_memory::<usize>(self.address + 0x8)?)
    }
}