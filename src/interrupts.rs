use crate::{gdt, println};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // TABLE OF CPU EXCEPTIONS
        // Name                               Vector nr.         Type        Mnemonic  Error code?
        // Divide-by-zero Error               0 (0x0)            Fault       #DE       No
        // Debug                              1 (0x1)            Fault/Trap  #DB       No
        // Non-maskable Interrupt             2 (0x2)            Interrupt   -         No
        // Breakpoint                         3 (0x3)            Trap        #BP       No
        // Overflow                           4 (0x4)            Trap        #OF       No
        // Bound Range Exceeded               5 (0x5)            Fault       #BR       No
        // Invalid Opcode                     6 (0x6)            Fault       #UD       No
        // Device Not Available               7 (0x7)            Fault       #NM       No
        // Double Fault                       8 (0x8)            Abort       #DF       Yes (Zero)
        // ~~ Coprocessor Segment Overrun     9 (0x9)            Fault       -         No   ~~
        // Invalid TSS                        10 (0xA)           Fault       #TS       Yes
        // Segment Not Present                11 (0xB)           Fault       #NP       Yes
        // Stack-Segment Fault                12 (0xC)           Fault       #SS       Yes
        // General Protection Fault           13 (0xD)           Fault       #GP       Yes
        // Page Fault                         14 (0xE)           Fault       #PF       Yes
        // ~~ Reserved                        15 (0xF)           -           -         No   ~~
        // x87 Floating-Point Exception       16 (0x10)          Fault       #MF       No
        // Alignment Check                    17 (0x11)          Fault       #AC       Yes
        // Machine Check                      18 (0x12)          Abort       #MC       No
        // SIMD Floating-Point Exception      19 (0x13)          Fault       #XM/#XF   No
        // Virtualization Exception           20 (0x14)          Fault       #VE       No
        // ~~ Control Protection Exception    21 (0x15)          Fault       #CP       Yes  ~~
        // ~~ Reserved                        22-27 (0x16-0x1B)  -           -         No   ~~
        // ~~ Hypervisor Injection Exception  28 (0x1C)          Fault       #HV       No   ~~
        // VMM Communication Exception        29 (0x1D)          Fault       #VC       Yes
        // Security Exception                 30 (0x1E)          Fault       #SX       Yes
        // ~~ Reserved                        31 (0x1F)          -           -         No   ~~
        // Triple Fault                       -                  -           -         No
        // ~~ FPU Error Interrupt             IRQ 13             Interrupt   #FERR     No   ~~


        // LIST OF CPU EXCEPTION HANDLERS

        // Divide-by-zero Error
        idt.divide_error.set_handler_fn(divide_by_zero_handler);

        // Debug
        idt.debug.set_handler_fn(debug_handler);

        // Non-maskable Interrupt
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_handler);

        // Breakpoint
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        // Overflow
        idt.overflow.set_handler_fn(overflow_handler);

        // Bound Range Exceeded
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);

        // Invalid Opcode
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);

        // Device Not Available
        idt.device_not_available.set_handler_fn(device_not_available_handler);

        // Double Fault
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // Invalid TSS
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);

        // Segment Not Present
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);

        // Stack-Segment Fault
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);

        // General Protection Fault
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);

        // Page Fault
        idt.page_fault.set_handler_fn(page_fault_handler);

        // x87 Floating-Point Exception
        idt.x87_floating_point.set_handler_fn(fpu_fault_handler);

        // Alignment Check
        idt.alignment_check.set_handler_fn(alignment_check_handler);

        // Machine Check
        idt.machine_check.set_handler_fn(machine_check_handler);

        // SIMD Floating-Point Exception
        idt.simd_floating_point.set_handler_fn(simd_handler);

        // Virtualization Exception
        idt.virtualization.set_handler_fn(virtualization_fault_handler);

        // VMM Communication Exception
        idt.vmm_communication_exception.set_handler_fn(vmm_communication_handler);

        // Security Exception
        idt.security_exception.set_handler_fn(security_handler);


        // LIST OF HARDWARE INTERRUPT HANDLERS

        // Timer
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);

        // Keyboard
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

// CPU EXCEPTION HANDLERS

// Divide-by-zero Error
extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) {
    println!("Divide by zero");
    dump(stack_frame);
}

// Debug
extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    println!("Debug trap");
    dump(stack_frame);
}

// Non-maskable Interrupt
extern "x86-interrupt" fn non_maskable_handler(stack_frame: InterruptStackFrame) {
    println!("Non-maskable interrupt");
    dump(stack_frame);
}

// Breakpoint
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("Breakpoint trap");
    dump(stack_frame);
}

// Overflow
extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    println!("Overflow trap");
    dump(stack_frame);
}

// Bound Range Exceeded
extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: InterruptStackFrame) {
    println!("Bound range exceeded fault");
    dump(stack_frame);
}

// Invalid Opcode
extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    println!("Invalid opcode fault");
    dump(stack_frame);
}

// Device Not Available
extern "x86-interrupt" fn device_not_available_handler(stack_frame: InterruptStackFrame) {
    println!("Device not available fault");
    dump(stack_frame);
}

// Double Fault
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "Double fault\nError code:{}\n{:#?}",
        error_code, stack_frame
    );
}

// Invalid TSS
extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println!("Invalid TSS fault");
    println!("Error code: {}", error_code);
    dump(stack_frame);
}

// Segment Not Present
extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("Segment not present fault");
    println!("Error code: {}", error_code);
    dump(stack_frame);
}

// Stack-Segment Fault
extern "x86-interrupt" fn stack_segment_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("Stack segment fault");
    println!("Error code: {}", error_code);
    dump(stack_frame);
}

// General Protection Fault
extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("General protection fault");
    println!("Error code: {}", error_code);
    dump(stack_frame);
}

// Page Fault
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("Page fault");
    println!("Accessed address: {:?}", Cr2::read());
    println!("Error code: {:?}", error_code);
    dump(stack_frame);
}

// x87 Floating-Point Exception
extern "x86-interrupt" fn fpu_fault_handler(stack_frame: InterruptStackFrame) {
    println!("FPU floating point fault");
    dump(stack_frame);
}

// Alignment Check
extern "x86-interrupt" fn alignment_check_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("Alignment check fault");
    println!("Error code: {}", error_code);
    dump(stack_frame);
}

// Machine Check
extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    panic!("Machine check fault\n{:#?}", stack_frame);
}

// SIMD Floating-Point Exception
extern "x86-interrupt" fn simd_handler(stack_frame: InterruptStackFrame) {
    println!("SIMD floating point fault");
    dump(stack_frame);
}

// Virtualization Exception
extern "x86-interrupt" fn virtualization_fault_handler(stack_frame: InterruptStackFrame) {
    println!("Virtualization fault");
    dump(stack_frame);
}

// VMM Communication Exception
extern "x86-interrupt" fn vmm_communication_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("Vmm communication");
    println!("Error code: {}", error_code);
    dump(stack_frame);
}

// Security Exception
extern "x86-interrupt" fn security_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println!("Security exception");
    println!("Error code: {}", error_code);
    dump(stack_frame);
}

// HARDWARE INTERRUPT HANDLERS

// timer
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

// keyboard
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

fn dump(stack_frame: InterruptStackFrame) {
    println!("{:#?}", stack_frame)
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
