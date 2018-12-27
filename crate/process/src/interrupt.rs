#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub unsafe fn disable_and_store() -> usize {
    let rflags: usize;
    asm!("pushfq; popq $0; cli" : "=r"(rflags));
    rflags & (1 << 9)
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub unsafe fn restore(flags: usize) {
    if flags != 0 {
        asm!("sti");
    }
}

#[inline(always)]
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub unsafe fn disable_and_store() -> usize {
    if option_env!("m_mode").is_some() {
        let mstatus: usize;
        asm!("csrci mstatus, 1 << 3" : "=r"(mstatus));
        mstatus & (1 << 3)
    } else {
        let sstatus: usize;
        asm!("csrci sstatus, 1 << 1" : "=r"(sstatus));
        sstatus & (1 << 1)
    }
}

#[inline(always)]
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub unsafe fn restore(flags: usize) {
    if option_env!("m_mode").is_some() {
        asm!("csrs mstatus, $0" :: "r"(flags));
    } else {
        asm!("csrs sstatus, $0" :: "r"(flags));
    }
}

#[inline(always)]
#[cfg(target_arch = "aarch64")]
pub unsafe fn disable_and_store() -> usize {
    let daif: u32;
    asm!("mrs $0, DAIF": "=r"(daif) ::: "volatile");
    asm!("msr daifset, #2");
    daif as usize
}

#[inline(always)]
#[cfg(target_arch = "aarch64")]
pub unsafe fn restore(flags: usize) {
    asm!("msr DAIF, $0" :: "r"(flags as u32) :: "volatile");
}
