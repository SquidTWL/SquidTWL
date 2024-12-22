use core::arch::asm;

/**
 * Wraps details about the *Current Program Status Register*.
 */
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CurrentProgramState(u32);

/**
 * The current mode the processor is in. Only System, Supervisor, and IRQ are used for NTR/TWL.
 */
#[derive(PartialEq, Eq, Debug)]
pub enum ProcessorMode {
    User,
    FIQ,
    IRQ,
    Supervisor,
    Abort,
    Undefined,
    System,
}

impl ProcessorMode {
    pub fn from_cpsr_bits(bits: u32) -> ProcessorMode {
        match bits & 0b11111 {
            0b10000 => ProcessorMode::User,
            0b10001 => ProcessorMode::FIQ,
            0b10010 => ProcessorMode::IRQ,
            0b10011 => ProcessorMode::Supervisor,
            0b10111 => ProcessorMode::Abort,
            0b11011 => ProcessorMode::Undefined,
            0b11111 => ProcessorMode::System,
            _ => panic!("unexpected mode! pass me an actual cpsr value!"),
        }
    }
}

impl CurrentProgramState {
    pub fn get() -> CurrentProgramState {
        return CurrentProgramState(get_raw_cpsr());
    }

    pub fn n(&self) -> bool {
        return (self.0 & 1 << 31) != 0;
    }
    pub fn z(&self) -> bool {
        return (self.0 & 1 << 30) != 0;
    }
    pub fn c(&self) -> bool {
        return (self.0 & 1 << 29) != 0;
    }
    pub fn v(&self) -> bool {
        return (self.0 & 1 << 28) != 0;
    }
    pub fn q(&self) -> bool {
        return (self.0 & 1 << 27) != 0;
    }

    pub fn interrupts_disabled(&self) -> bool {
        return (self.0 & (1 << 7)) != 0;
    }

    pub fn mode(&self) -> ProcessorMode {
        return ProcessorMode::from_cpsr_bits(self.0 & 0b11111);
    }
}

/**
 * Gets the raw CPSR value.
 */
pub fn get_raw_cpsr() -> u32 {
    let mut cpsr: u32;
    unsafe {
        asm!("mrs {}, cpsr", out(reg) cpsr);
    };

    return cpsr;
}
