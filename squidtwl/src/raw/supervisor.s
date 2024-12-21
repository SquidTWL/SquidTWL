.section .text

// See the definitions in ``supervisor.rs`` for more information.
// GBATEK says:
// Caution: When invoking SWIs from inside of ARM state specify SWI NN*10000h, instead of 
// SWI NN as in THUMB state.
//
// We say: Don't invoke SWIs in Thumb as it doesn't work properly in the emulator?

.global SWI_Halt
SWI_Halt:
    swi #0x60000
    bx lr
