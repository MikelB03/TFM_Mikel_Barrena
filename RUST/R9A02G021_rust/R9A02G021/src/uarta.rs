#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    txba: (),
    _reserved1: [u8; 0x01],
    rxba: (),
    _reserved2: [u8; 0x01],
    asima0: (),
    _reserved3: [u8; 0x01],
    asima1: (),
    _reserved4: [u8; 0x01],
    brgca: (),
    _reserved5: [u8; 0x01],
    asisa: (),
    _reserved6: [u8; 0x01],
    ascta: (),
    _reserved7: [u8; 0x0a],
    uta0ck: Uta0ck,
    uta1ck: Uta1ck,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit Buffer Register %s"]
    #[inline(always)]
    pub const fn txba(&self, n: usize) -> &Txba {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - Transmit Buffer Register %s"]
    #[inline(always)]
    pub fn txba_iter(&self) -> impl Iterator<Item = &Txba> {
        (0..2).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8 * n).cast() })
    }
    #[doc = "0x01 - Receive Buffer Register %s"]
    #[inline(always)]
    pub const fn rxba(&self, n: usize) -> &Rxba {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x01 - Receive Buffer Register %s"]
    #[inline(always)]
    pub fn rxba_iter(&self) -> impl Iterator<Item = &Rxba> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x02 - Operation Mode Setting Register n0"]
    #[inline(always)]
    pub const fn asima0(&self, n: usize) -> &Asima0 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x02 - Operation Mode Setting Register n0"]
    #[inline(always)]
    pub fn asima0_iter(&self) -> impl Iterator<Item = &Asima0> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x02 - Operation Mode Setting Register n0"]
    #[inline(always)]
    pub const fn asima00(&self) -> &Asima0 {
        self.asima0(0)
    }
    #[doc = "0x0a - Operation Mode Setting Register n0"]
    #[inline(always)]
    pub const fn asima10(&self) -> &Asima0 {
        self.asima0(1)
    }
    #[doc = "0x03 - Operation Mode Setting Register n1"]
    #[inline(always)]
    pub const fn asima1(&self, n: usize) -> &Asima1 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(3)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x03 - Operation Mode Setting Register n1"]
    #[inline(always)]
    pub fn asima1_iter(&self) -> impl Iterator<Item = &Asima1> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(3)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x03 - Operation Mode Setting Register n1"]
    #[inline(always)]
    pub const fn asima01(&self) -> &Asima1 {
        self.asima1(0)
    }
    #[doc = "0x0b - Operation Mode Setting Register n1"]
    #[inline(always)]
    pub const fn asima11(&self) -> &Asima1 {
        self.asima1(1)
    }
    #[doc = "0x04 - Baud Rate Generator Control Register %s"]
    #[inline(always)]
    pub const fn brgca(&self, n: usize) -> &Brgca {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04 - Baud Rate Generator Control Register %s"]
    #[inline(always)]
    pub fn brgca_iter(&self) -> impl Iterator<Item = &Brgca> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x05 - Status Register %s"]
    #[inline(always)]
    pub const fn asisa(&self, n: usize) -> &Asisa {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x05 - Status Register %s"]
    #[inline(always)]
    pub fn asisa_iter(&self) -> impl Iterator<Item = &Asisa> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x06 - Status Clear Trigger Register %s"]
    #[inline(always)]
    pub const fn ascta(&self, n: usize) -> &Ascta {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x06 - Status Clear Trigger Register %s"]
    #[inline(always)]
    pub fn ascta_iter(&self) -> impl Iterator<Item = &Ascta> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x10 - UARTA Clock Select Register 0"]
    #[inline(always)]
    pub const fn uta0ck(&self) -> &Uta0ck {
        &self.uta0ck
    }
    #[doc = "0x11 - UARTA Clock Select Register 1"]
    #[inline(always)]
    pub const fn uta1ck(&self) -> &Uta1ck {
        &self.uta1ck
    }
}
#[doc = "TXBA (rw) register accessor: Transmit Buffer Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`txba::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txba::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txba`]
module"]
#[doc(alias = "TXBA")]
pub type Txba = crate::Reg<txba::TxbaSpec>;
#[doc = "Transmit Buffer Register %s"]
pub mod txba;
#[doc = "RXBA (r) register accessor: Receive Buffer Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rxba::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxba`]
module"]
#[doc(alias = "RXBA")]
pub type Rxba = crate::Reg<rxba::RxbaSpec>;
#[doc = "Receive Buffer Register %s"]
pub mod rxba;
#[doc = "ASIMA0 (rw) register accessor: Operation Mode Setting Register n0\n\nYou can [`read`](crate::Reg::read) this register and get [`asima0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asima0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asima0`]
module"]
#[doc(alias = "ASIMA0")]
pub type Asima0 = crate::Reg<asima0::Asima0Spec>;
#[doc = "Operation Mode Setting Register n0"]
pub mod asima0;
#[doc = "ASIMA1 (rw) register accessor: Operation Mode Setting Register n1\n\nYou can [`read`](crate::Reg::read) this register and get [`asima1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asima1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asima1`]
module"]
#[doc(alias = "ASIMA1")]
pub type Asima1 = crate::Reg<asima1::Asima1Spec>;
#[doc = "Operation Mode Setting Register n1"]
pub mod asima1;
#[doc = "BRGCA (rw) register accessor: Baud Rate Generator Control Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`brgca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brgca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brgca`]
module"]
#[doc(alias = "BRGCA")]
pub type Brgca = crate::Reg<brgca::BrgcaSpec>;
#[doc = "Baud Rate Generator Control Register %s"]
pub mod brgca;
#[doc = "ASISA (r) register accessor: Status Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`asisa::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asisa`]
module"]
#[doc(alias = "ASISA")]
pub type Asisa = crate::Reg<asisa::AsisaSpec>;
#[doc = "Status Register %s"]
pub mod asisa;
#[doc = "ASCTA (rw) register accessor: Status Clear Trigger Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ascta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ascta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ascta`]
module"]
#[doc(alias = "ASCTA")]
pub type Ascta = crate::Reg<ascta::AsctaSpec>;
#[doc = "Status Clear Trigger Register %s"]
pub mod ascta;
#[doc = "UTA0CK (rw) register accessor: UARTA Clock Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uta0ck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uta0ck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uta0ck`]
module"]
#[doc(alias = "UTA0CK")]
pub type Uta0ck = crate::Reg<uta0ck::Uta0ckSpec>;
#[doc = "UARTA Clock Select Register 0"]
pub mod uta0ck;
#[doc = "UTA1CK (rw) register accessor: UARTA Clock Select Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uta1ck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uta1ck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uta1ck`]
module"]
#[doc(alias = "UTA1CK")]
pub type Uta1ck = crate::Reg<uta1ck::Uta1ckSpec>;
#[doc = "UARTA Clock Select Register 1"]
pub mod uta1ck;
