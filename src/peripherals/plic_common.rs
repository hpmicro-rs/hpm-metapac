#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PLIC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plic {
    ptr: *mut u8,
}
unsafe impl Send for Plic {}
unsafe impl Sync for Plic {}
impl Plic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Feature enable register."]
    #[inline(always)]
    pub const fn feature(self) -> crate::common::Reg<regs::Feature, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn priority(self, n: usize) -> crate::common::Reg<regs::Priority, crate::common::RW> {
        assert!(n < 127usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pending(self, n: usize) -> crate::common::Reg<regs::Pending, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trigger(self, n: usize) -> crate::common::Reg<regs::Trigger, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1080usize + n * 4usize) as _) }
    }
    #[doc = "Number of supported interrupt sources and targets."]
    #[inline(always)]
    pub const fn number(self) -> crate::common::Reg<regs::Number, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1100usize) as _) }
    }
    #[doc = "Version and the maximum priority."]
    #[inline(always)]
    pub const fn info(self) -> crate::common::Reg<regs::Info, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1104usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn targetint(self, n: usize) -> Targetint {
        assert!(n < 2usize);
        unsafe { Targetint::from_ptr(self.ptr.add(0x2000usize + n * 128usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn targetconfig(self, n: usize) -> Targetconfig {
        assert!(n < 2usize);
        unsafe { Targetconfig::from_ptr(self.ptr.add(0x0020_0000usize + n * 4096usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Targetconfig {
    ptr: *mut u8,
}
unsafe impl Send for Targetconfig {}
unsafe impl Sync for Targetconfig {}
impl Targetconfig {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Target0 priority threshold."]
    #[inline(always)]
    pub const fn threshold(self) -> crate::common::Reg<regs::Threshold, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Target claim and complete."]
    #[inline(always)]
    pub const fn claim(self) -> crate::common::Reg<regs::Claim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Preempted priority stack."]
    #[inline(always)]
    pub const fn pps(self) -> crate::common::Reg<regs::Pps, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Targetint {
    ptr: *mut u8,
}
unsafe impl Send for Targetint {}
unsafe impl Sync for Targetint {}
impl Targetint {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn inten(self, n: usize) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Target claim and complete."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Claim(pub u32);
    impl Claim {
        #[doc = "On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
        #[inline(always)]
        pub const fn interrupt_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
        #[inline(always)]
        pub fn set_interrupt_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Claim {
        #[inline(always)]
        fn default() -> Claim {
            Claim(0)
        }
    }
    #[doc = "Feature enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Feature(pub u32);
    impl Feature {
        #[doc = "Preemptive priority interrupt enable 0: Disabled 1: Enabled."]
        #[inline(always)]
        pub const fn preempt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Preemptive priority interrupt enable 0: Disabled 1: Enabled."]
        #[inline(always)]
        pub fn set_preempt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Vector mode enable 0: Disabled 1: Enabled."]
        #[inline(always)]
        pub const fn vectored(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Vector mode enable 0: Disabled 1: Enabled."]
        #[inline(always)]
        pub fn set_vectored(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Feature {
        #[inline(always)]
        fn default() -> Feature {
            Feature(0)
        }
    }
    #[doc = "Version and the maximum priority."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Info(pub u32);
    impl Info {
        #[doc = "The version of the PLIC design."]
        #[inline(always)]
        pub const fn version(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The version of the PLIC design."]
        #[inline(always)]
        pub fn set_version(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The maximum priority supported."]
        #[inline(always)]
        pub const fn max_priority(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The maximum priority supported."]
        #[inline(always)]
        pub fn set_max_priority(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Info {
        #[inline(always)]
        fn default() -> Info {
            Info(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inten(pub u32);
    impl Inten {
        #[doc = "The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
        #[inline(always)]
        pub const fn interrupt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
        #[inline(always)]
        pub fn set_interrupt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Inten {
        #[inline(always)]
        fn default() -> Inten {
            Inten(0)
        }
    }
    #[doc = "Number of supported interrupt sources and targets."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Number(pub u32);
    impl Number {
        #[doc = "The number of supported interrupt sources."]
        #[inline(always)]
        pub const fn num_interrupt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The number of supported interrupt sources."]
        #[inline(always)]
        pub fn set_num_interrupt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The number of supported targets."]
        #[inline(always)]
        pub const fn num_target(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The number of supported targets."]
        #[inline(always)]
        pub fn set_num_target(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Number {
        #[inline(always)]
        fn default() -> Number {
            Number(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pending(pub u32);
    impl Pending {
        #[doc = "The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit."]
        #[inline(always)]
        pub const fn interrupt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit."]
        #[inline(always)]
        pub fn set_interrupt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pending {
        #[inline(always)]
        fn default() -> Pending {
            Pending(0)
        }
    }
    #[doc = "Preempted priority stack."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pps(pub u32);
    impl Pps {
        #[doc = "Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
        #[inline(always)]
        pub const fn priority_preempted(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
        #[inline(always)]
        pub fn set_priority_preempted(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pps {
        #[inline(always)]
        fn default() -> Pps {
            Pps(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priority(pub u32);
    impl Priority {
        #[doc = "Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
        #[inline(always)]
        pub const fn priority(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
        #[inline(always)]
        pub fn set_priority(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Priority {
        #[inline(always)]
        fn default() -> Priority {
            Priority(0)
        }
    }
    #[doc = "Target0 priority threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Threshold(pub u32);
    impl Threshold {
        #[doc = "Interrupt priority threshold."]
        #[inline(always)]
        pub const fn threshold(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interrupt priority threshold."]
        #[inline(always)]
        pub fn set_threshold(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Threshold {
        #[inline(always)]
        fn default() -> Threshold {
            Threshold(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trigger(pub u32);
    impl Trigger {
        #[doc = "The interrupt trigger type of interrupt sources. Every interrupt source occupies 1 bit. 0: Level-triggered interrupt 1: Edge-triggered interrupt."]
        #[inline(always)]
        pub const fn interrupt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The interrupt trigger type of interrupt sources. Every interrupt source occupies 1 bit. 0: Level-triggered interrupt 1: Edge-triggered interrupt."]
        #[inline(always)]
        pub fn set_interrupt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Trigger {
        #[inline(always)]
        fn default() -> Trigger {
            Trigger(0)
        }
    }
}
