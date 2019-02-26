//! Contains EVM-specific types

pub use arch::arch_builder::evm::*;
use arch::DetailsArchInsn;
use capstone_sys::cs_evm;
use std::fmt;

// XXX todo(tmfink): create rusty versions
pub use capstone_sys::evm_insn_group as EvmInsnGroup;
pub use capstone_sys::evm_insn as EvmInsn;
// no register type since EVM has no operands

/// Contains EVM-specific details for an instruction
pub struct EvmInsnDetail<'a>(pub(crate) &'a cs_evm);

impl<'a> EvmInsnDetail<'a> {
    /// Number of items popped from the stack
    pub fn popped_items(&self) -> u8 {
        self.0.pop
    }

    /// Number of items pushed into the stack
    pub fn pushed_items(&self) -> u8 {
        self.0.push
    }

    /// Gas fee for the instruction
    pub fn fee(&self) -> u32 {
        self.0.fee as u32
    }
}

impl_PartialEq_repr_fields!(EvmInsnDetail<'a> [ 'a ];
    popped_items, pushed_items, fee
);

/// EVM has no operands, so this is a zero-size type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvmOperand(());

impl Default for EvmOperand {
    fn default() -> Self {
        EvmOperand(())
    }
}

// Do not use def_arch_details_struct! since EVM does not have operands

/// Iterates over instruction operands
#[derive(Clone)]
pub struct EvmOperandIterator(());

impl EvmOperandIterator {
    fn new() -> EvmOperandIterator {
        EvmOperandIterator(())
    }
}

impl Iterator for EvmOperandIterator {
    type Item = EvmOperand;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl ExactSizeIterator for EvmOperandIterator {
    fn len(&self) -> usize {
        0
    }
}

impl PartialEq for EvmOperandIterator {
    fn eq(&self, _other: &EvmOperandIterator) -> bool {
        false
    }
}

impl fmt::Debug for EvmOperandIterator {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::fmt::Result {
        fmt.debug_struct("EvmOperandIterator").finish()
    }
}

impl<'a> fmt::Debug for EvmInsnDetail<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::fmt::Result {
        fmt.debug_struct("EvmInsnDetail")
            .field("cs_evm", &(self.0 as *const cs_evm))
            .finish()
    }
}

impl<'a> DetailsArchInsn for EvmInsnDetail<'a> {
    type OperandIterator = EvmOperandIterator;
    type Operand = EvmOperand;

    fn operands(&self) -> EvmOperandIterator {
        EvmOperandIterator::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_evm_detail() {
        let cs_evm = cs_evm {
            pop: 1,
            push: 2,
            fee: 42,
        };
        let d = EvmInsnDetail(&cs_evm);
        assert_eq!(d.popped_items(), 1);
        assert_eq!(d.pushed_items(), 2);
        assert_eq!(d.fee(), 42);
    }
}
