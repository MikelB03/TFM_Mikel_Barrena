#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
p00pfs: [P00pfs; 10],
    _reserved1: [u8; 0x1000],
    p00pfs_ha: (),
    _reserved2: [u8; 0x01],
    p00pfs_by: (),
    _reserved3: [u8; 0x25],
    p0pfs: [P0pfs; 2],
    _reserved4: [u8; 0x1000],
    p0pfs_ha: (),
    _reserved5: [u8; 0x01],
    p0pfs_by: (),
    _reserved6: [u8; 0x15],
    p10pfs: [P10pfs; 10],
    _reserved7: [u8; 0x1000],
    p10pfs_ha: (),
    _reserved8: [u8; 0x01],
    p10pfs_by: (),
    _reserved9: [u8; 0x25],
    p1pfs: [P1pfs; 2],
    _reserved10: [u8; 0x1000],
    p1pfs_ha: (),
    _reserved11: [u8; 0x01],
    p1pfs_by: (),
    _reserved12: [u8; 0x15],
    p20pfs: [P20pfs; 3],
    _reserved13: [u8; 0x1000],
    p20pfs_ha: (),
    _reserved14: [u8; 0x01],
    p20pfs_by: (),
    _reserved15: [u8; 0x09],
    _reserved_15_p203: [u8; 0x04],
    _reserved_16_p204: [u8; 0x04],
    _reserved_17_p205: [u8; 0x04],
    _reserved_18_p206: [u8; 0x04],
    _reserved_19_p207: [u8; 0x04],
    _reserved20: [u8; 0x20],
    _reserved_20_p300: [u8; 0x04],
    _reserved_21_p301: [u8; 0x04],
    _reserved_22_p302: [u8; 0x04],
    _reserved_23_p303: [u8; 0x04],
    p30pfs: [P30pfs; 4],
    _reserved25: [u8; 0x1000],
    p30pfs_ha: (),
    _reserved26: [u8; 0x01],
    p30pfs_by: (),
    _reserved27: [u8; 0x2d],
    p40pfs: [P40pfs; 4],
    _reserved28: [u8; 0x1000],
    p40pfs_ha: (),
    _reserved29: [u8; 0x01],
    p40pfs_by: (),
    _reserved30: [u8; 0x0400],
    pwpr: Pwpr,
    _reserved31: [u8; 0x0b],
    prwcntr: Prwcntr,
}
impl RegisterBlock {
    #[doc = "0x00..0x28 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs(&self, n: usize) -> &P00pfs {
        &self.p00pfs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x28 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p00pfs_iter(&self) -> impl Iterator<Item = &P00pfs> {
        self.p00pfs.iter()
    }
    #[doc = "0x00 - Port 000 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p000pfs(&self) -> &P00pfs {
        self.p00pfs(0)
    }
    #[doc = "0x04 - Port 001 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p001pfs(&self) -> &P00pfs {
        self.p00pfs(1)
    }
    #[doc = "0x08 - Port 002 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p002pfs(&self) -> &P00pfs {
        self.p00pfs(2)
    }
    #[doc = "0x0c - Port 003 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p003pfs(&self) -> &P00pfs {
        self.p00pfs(3)
    }
    #[doc = "0x10 - Port 004 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p004pfs(&self) -> &P00pfs {
        self.p00pfs(4)
    }
    #[doc = "0x14 - Port 005 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p005pfs(&self) -> &P00pfs {
        self.p00pfs(5)
    }
    #[doc = "0x18 - Port 006 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p006pfs(&self) -> &P00pfs {
        self.p00pfs(6)
    }
    #[doc = "0x1c - Port 007 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p007pfs(&self) -> &P00pfs {
        self.p00pfs(7)
    }
    #[doc = "0x20 - Port 008 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p008pfs(&self) -> &P00pfs {
        self.p00pfs(8)
    }
    #[doc = "0x24 - Port 009 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p009pfs(&self) -> &P00pfs {
        self.p00pfs(9)
    }
    #[doc = "0x02..0x16 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs_ha(&self, n: usize) -> &P00pfsHa {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x02..0x16 - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p00pfs_ha_iter(&self) -> impl Iterator<Item = &P00pfsHa> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x02 - Port 000 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p000pfs_ha(&self) -> &P00pfsHa {
        self.p00pfs_ha(0)
    }
    #[doc = "0x06 - Port 001 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p001pfs_ha(&self) -> &P00pfsHa {
        self.p00pfs_ha(1)
    }
    #[doc = "0x0a - Port 002 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p002pfs_ha(&self) -> &P00pfsHa {
        self.p00pfs_ha(2)
    }
    #[doc = "0x0e - Port 003 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p003pfs_ha(&self) -> &P00pfsHa {
        self.p00pfs_ha(3)
    }
    #[doc = "0x12 - Port 004 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p004pfs_ha(&self) -> &P00pfsHa {
        self.p00pfs_ha(4)
    }
    #[doc = "0x16 - Port 005 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p005pfs_ha(&self) -> &P00pfsHa {
        self.p00pfs_ha(5)
    }
    #[doc = "0x1a - Port 006 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p006pfs_ha(&self) -> &P00pfsHa {
        self.p00pfs_ha(6)
    }
    #[doc = "0x1e - Port 007 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p007pfs_ha(&self) -> &P00pfsHa {
        self.p00pfs_ha(7)
    }
    #[doc = "0x22 - Port 008 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p008pfs_ha(&self) -> &P00pfsHa {
        self.p00pfs_ha(8)
    }
    #[doc = "0x26 - Port 009 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p009pfs_ha(&self) -> &P00pfsHa {
        self.p00pfs_ha(9)
    }
    #[doc = "0x03..0x0d - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs_by(&self, n: usize) -> &P00pfsBy {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(3)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x03..0x0d - Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p00pfs_by_iter(&self) -> impl Iterator<Item = &P00pfsBy> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(3)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x03 - Port 000 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p000pfs_by(&self) -> &P00pfsBy {
        self.p00pfs_by(0)
    }
    #[doc = "0x07 - Port 001 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p001pfs_by(&self) -> &P00pfsBy {
        self.p00pfs_by(1)
    }
    #[doc = "0x0b - Port 002 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p002pfs_by(&self) -> &P00pfsBy {
        self.p00pfs_by(2)
    }
    #[doc = "0x0f - Port 003 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p003pfs_by(&self) -> &P00pfsBy {
        self.p00pfs_by(3)
    }
    #[doc = "0x13 - Port 004 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p004pfs_by(&self) -> &P00pfsBy {
        self.p00pfs_by(4)
    }
    #[doc = "0x17 - Port 005 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p005pfs_by(&self) -> &P00pfsBy {
        self.p00pfs_by(5)
    }
    #[doc = "0x1b - Port 006 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p006pfs_by(&self) -> &P00pfsBy {
        self.p00pfs_by(6)
    }
    #[doc = "0x1f - Port 007 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p007pfs_by(&self) -> &P00pfsBy {
        self.p00pfs_by(7)
    }
    #[doc = "0x23 - Port 008 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p008pfs_by(&self) -> &P00pfsBy {
        self.p00pfs_by(8)
    }
    #[doc = "0x27 - Port 009 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p009pfs_by(&self) -> &P00pfsBy {
        self.p00pfs_by(9)
    }
    #[doc = "0x28..0x30 - Port 0%s Pin Function Select Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `P010PFS` register.</div>"]
    #[inline(always)]
    pub const fn p0pfs(&self, n: usize) -> &P0pfs {
        &self.p0pfs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28..0x30 - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p0pfs_iter(&self) -> impl Iterator<Item = &P0pfs> {
        self.p0pfs.iter()
    }
    #[doc = "0x28 - Port 010 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p010pfs(&self) -> &P0pfs {
        self.p0pfs(0)
    }
    #[doc = "0x2c - Port 011 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p011pfs(&self) -> &P0pfs {
        self.p0pfs(1)
    }
    #[doc = "0x2a - Port 0%s Pin Function Select Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `P010PFS_HA` register.</div>"]
    #[inline(always)]
    pub const fn p0pfs_ha(&self, n: usize) -> &P0pfsHa {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(42)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2a - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p0pfs_ha_iter(&self) -> impl Iterator<Item = &P0pfsHa> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(42)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x2a - Port 010 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p010pfs_ha(&self) -> &P0pfsHa {
        self.p0pfs_ha(0)
    }
    #[doc = "0x2e - Port 011 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p011pfs_ha(&self) -> &P0pfsHa {
        self.p0pfs_ha(1)
    }
    #[doc = "0x2b - Port 0%s Pin Function Select Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `P010PFS_BY` register.</div>"]
    #[inline(always)]
    pub const fn p0pfs_by(&self, n: usize) -> &P0pfsBy {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(43)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2b - Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p0pfs_by_iter(&self) -> impl Iterator<Item = &P0pfsBy> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(43)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x2b - Port 010 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p010pfs_by(&self) -> &P0pfsBy {
        self.p0pfs_by(0)
    }
    #[doc = "0x2f - Port 011 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p011pfs_by(&self) -> &P0pfsBy {
        self.p0pfs_by(1)
    }
    #[doc = "0x40..0x68 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs(&self, n: usize) -> &P10pfs {
        &self.p10pfs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x68 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p10pfs_iter(&self) -> impl Iterator<Item = &P10pfs> {
        self.p10pfs.iter()
    }
    #[doc = "0x40 - Port 100 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p100pfs(&self) -> &P10pfs {
        self.p10pfs(0)
    }
    #[doc = "0x44 - Port 101 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p101pfs(&self) -> &P10pfs {
        self.p10pfs(1)
    }
    #[doc = "0x48 - Port 102 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p102pfs(&self) -> &P10pfs {
        self.p10pfs(2)
    }
    #[doc = "0x4c - Port 103 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p103pfs(&self) -> &P10pfs {
        self.p10pfs(3)
    }
    #[doc = "0x50 - Port 104 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p104pfs(&self) -> &P10pfs {
        self.p10pfs(4)
    }
    #[doc = "0x54 - Port 105 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p105pfs(&self) -> &P10pfs {
        self.p10pfs(5)
    }
    #[doc = "0x58 - Port 106 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p106pfs(&self) -> &P10pfs {
        self.p10pfs(6)
    }
    #[doc = "0x5c - Port 107 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p107pfs(&self) -> &P10pfs {
        self.p10pfs(7)
    }
    #[doc = "0x60 - Port 108 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p108pfs(&self) -> &P10pfs {
        self.p10pfs(8)
    }
    #[doc = "0x64 - Port 109 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p109pfs(&self) -> &P10pfs {
        self.p10pfs(9)
    }
    #[doc = "0x42..0x56 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs_ha(&self, n: usize) -> &P10pfsHa {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(66)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x42..0x56 - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p10pfs_ha_iter(&self) -> impl Iterator<Item = &P10pfsHa> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(66)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x42 - Port 100 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p100pfs_ha(&self) -> &P10pfsHa {
        self.p10pfs_ha(0)
    }
    #[doc = "0x46 - Port 101 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p101pfs_ha(&self) -> &P10pfsHa {
        self.p10pfs_ha(1)
    }
    #[doc = "0x4a - Port 102 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p102pfs_ha(&self) -> &P10pfsHa {
        self.p10pfs_ha(2)
    }
    #[doc = "0x4e - Port 103 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p103pfs_ha(&self) -> &P10pfsHa {
        self.p10pfs_ha(3)
    }
    #[doc = "0x52 - Port 104 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p104pfs_ha(&self) -> &P10pfsHa {
        self.p10pfs_ha(4)
    }
    #[doc = "0x56 - Port 105 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p105pfs_ha(&self) -> &P10pfsHa {
        self.p10pfs_ha(5)
    }
    #[doc = "0x5a - Port 106 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p106pfs_ha(&self) -> &P10pfsHa {
        self.p10pfs_ha(6)
    }
    #[doc = "0x5e - Port 107 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p107pfs_ha(&self) -> &P10pfsHa {
        self.p10pfs_ha(7)
    }
    #[doc = "0x62 - Port 108 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p108pfs_ha(&self) -> &P10pfsHa {
        self.p10pfs_ha(8)
    }
    #[doc = "0x66 - Port 109 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p109pfs_ha(&self) -> &P10pfsHa {
        self.p10pfs_ha(9)
    }
    #[doc = "0x43..0x4d - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs_by(&self, n: usize) -> &P10pfsBy {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(67)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x43..0x4d - Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p10pfs_by_iter(&self) -> impl Iterator<Item = &P10pfsBy> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(67)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x43 - Port 100 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p100pfs_by(&self) -> &P10pfsBy {
        self.p10pfs_by(0)
    }
    #[doc = "0x47 - Port 101 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p101pfs_by(&self) -> &P10pfsBy {
        self.p10pfs_by(1)
    }
    #[doc = "0x4b - Port 102 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p102pfs_by(&self) -> &P10pfsBy {
        self.p10pfs_by(2)
    }
    #[doc = "0x4f - Port 103 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p103pfs_by(&self) -> &P10pfsBy {
        self.p10pfs_by(3)
    }
    #[doc = "0x53 - Port 104 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p104pfs_by(&self) -> &P10pfsBy {
        self.p10pfs_by(4)
    }
    #[doc = "0x57 - Port 105 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p105pfs_by(&self) -> &P10pfsBy {
        self.p10pfs_by(5)
    }
    #[doc = "0x5b - Port 106 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p106pfs_by(&self) -> &P10pfsBy {
        self.p10pfs_by(6)
    }
    #[doc = "0x5f - Port 107 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p107pfs_by(&self) -> &P10pfsBy {
        self.p10pfs_by(7)
    }
    #[doc = "0x63 - Port 108 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p108pfs_by(&self) -> &P10pfsBy {
        self.p10pfs_by(8)
    }
    #[doc = "0x67 - Port 109 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p109pfs_by(&self) -> &P10pfsBy {
        self.p10pfs_by(9)
    }
    #[doc = "0x68..0x70 - Port 1%s Pin Function Select Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `P110PFS` register.</div>"]
    #[inline(always)]
    pub const fn p1pfs(&self, n: usize) -> &P1pfs {
        &self.p1pfs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x68..0x70 - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p1pfs_iter(&self) -> impl Iterator<Item = &P1pfs> {
        self.p1pfs.iter()
    }
    #[doc = "0x68 - Port 110 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p110pfs(&self) -> &P1pfs {
        self.p1pfs(0)
    }
    #[doc = "0x6c - Port 111 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p111pfs(&self) -> &P1pfs {
        self.p1pfs(1)
    }
    #[doc = "0x6a - Port 1%s Pin Function Select Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `P110PFS_HA` register.</div>"]
    #[inline(always)]
    pub const fn p1pfs_ha(&self, n: usize) -> &P1pfsHa {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(106)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x6a - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p1pfs_ha_iter(&self) -> impl Iterator<Item = &P1pfsHa> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(106)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x6a - Port 110 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p110pfs_ha(&self) -> &P1pfsHa {
        self.p1pfs_ha(0)
    }
    #[doc = "0x6e - Port 111 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p111pfs_ha(&self) -> &P1pfsHa {
        self.p1pfs_ha(1)
    }
    #[doc = "0x6b - Port 1%s Pin Function Select Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `P110PFS_BY` register.</div>"]
    #[inline(always)]
    pub const fn p1pfs_by(&self, n: usize) -> &P1pfsBy {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(107)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x6b - Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p1pfs_by_iter(&self) -> impl Iterator<Item = &P1pfsBy> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(107)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x6b - Port 110 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p110pfs_by(&self) -> &P1pfsBy {
        self.p1pfs_by(0)
    }
    #[doc = "0x6f - Port 111 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p111pfs_by(&self) -> &P1pfsBy {
        self.p1pfs_by(1)
    }
    #[doc = "0x80..0x8c - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs(&self, n: usize) -> &P20pfs {
        &self.p20pfs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x8c - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p20pfs_iter(&self) -> impl Iterator<Item = &P20pfs> {
        self.p20pfs.iter()
    }
    #[doc = "0x80 - Port 200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs(&self) -> &P20pfs {
        self.p20pfs(0)
    }
    #[doc = "0x84 - Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs(&self) -> &P20pfs {
        self.p20pfs(1)
    }
    #[doc = "0x88 - Port 202 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p202pfs(&self) -> &P20pfs {
        self.p20pfs(2)
    }
    #[doc = "0x82..0x88 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs_ha(&self, n: usize) -> &P20pfsHa {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(130)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x82..0x88 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p20pfs_ha_iter(&self) -> impl Iterator<Item = &P20pfsHa> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(130)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x82 - Port 200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs_ha(&self) -> &P20pfsHa {
        self.p20pfs_ha(0)
    }
    #[doc = "0x86 - Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs_ha(&self) -> &P20pfsHa {
        self.p20pfs_ha(1)
    }
    #[doc = "0x8a - Port 202 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p202pfs_ha(&self) -> &P20pfsHa {
        self.p20pfs_ha(2)
    }
    #[doc = "0x83 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs_by(&self, n: usize) -> &P20pfsBy {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(131)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x83 - Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p20pfs_by_iter(&self) -> impl Iterator<Item = &P20pfsBy> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(131)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x83 - Port 200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs_by(&self) -> &P20pfsBy {
        self.p20pfs_by(0)
    }
    #[doc = "0x87 - Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs_by(&self) -> &P20pfsBy {
        self.p20pfs_by(1)
    }
    #[doc = "0x8b - Port 202 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p202pfs_by(&self) -> &P20pfsBy {
        self.p20pfs_by(2)
    }
    #[doc = "0x8c - Port 203 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p203pfs(&self) -> &P203pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x8e - Port 203 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p203pfs_ha(&self) -> &P203pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(142).cast() }
    }
    #[doc = "0x8f - Port 203 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p203pfs_by(&self) -> &P203pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(143).cast() }
    }
    #[doc = "0x90 - Port 204 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p204pfs(&self) -> &P20pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x92 - Port 204 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p204pfs_ha(&self) -> &P20pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(146).cast() }
    }
    #[doc = "0x93 - Port 204 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p204pfs_by(&self) -> &P20pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(147).cast() }
    }
    #[doc = "0x90 - Port 205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs(&self) -> &P20pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x92 - Port 205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs_ha(&self) -> &P20pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(146).cast() }
    }
    #[doc = "0x93 - Port 205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs_by(&self) -> &P20pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(147).cast() }
    }
    #[doc = "0x90 - Port 206 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p206pfs(&self) -> &P20pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x92 - Port 206 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p206pfs_ha(&self) -> &P20pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(146).cast() }
    }
    #[doc = "0x93 - Port 206 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p206pfs_by(&self) -> &P20pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(147).cast() }
    }
    #[doc = "0x90 - Port 207 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p207pfs(&self) -> &P20pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x92 - Port 207 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p207pfs_ha(&self) -> &P20pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(146).cast() }
    }
    #[doc = "0x93 - Port 207 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p207pfs_by(&self) -> &P20pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(147).cast() }
    }
    #[doc = "0xc0 - Port 300 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p300pfs(&self) -> &P300pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(192).cast() }
    }
    #[doc = "0xc2 - Port 300 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p300pfs_ha(&self) -> &P300pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(194).cast() }
    }
    #[doc = "0xc3 - Port 300 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p300pfs_by(&self) -> &P300pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(195).cast() }
    }
    #[doc = "0xc4 - Port 301 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p301pfs(&self) -> &P301pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc6 - Port 301 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p301pfs_ha(&self) -> &P301pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    #[doc = "0xc7 - Port 301 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p301pfs_by(&self) -> &P301pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    #[doc = "0xc8 - Port 302 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p302pfs(&self) -> &P302pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xca - Port 302 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p302pfs_ha(&self) -> &P302pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(202).cast() }
    }
    #[doc = "0xcb - Port 302 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p302pfs_by(&self) -> &P302pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(203).cast() }
    }
    #[doc = "0xcc - Port 303 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p303pfs(&self) -> &P303pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xce - Port 303 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p303pfs_ha(&self) -> &P303pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(206).cast() }
    }
    #[doc = "0xcf - Port 303 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p303pfs_by(&self) -> &P303pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(207).cast() }
    }
    #[doc = "0xd0..0xe0 - Port 30%s Pin Function Select Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `P304PFS` register.</div>"]
    #[inline(always)]
    pub const fn p30pfs(&self, n: usize) -> &P30pfs {
        &self.p30pfs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd0..0xe0 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p30pfs_iter(&self) -> impl Iterator<Item = &P30pfs> {
        self.p30pfs.iter()
    }
    #[doc = "0xd0 - Port 304 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p304pfs(&self) -> &P30pfs {
        self.p30pfs(0)
    }
    #[doc = "0xd4 - Port 305 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p305pfs(&self) -> &P30pfs {
        self.p30pfs(1)
    }
    #[doc = "0xd8 - Port 306 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p306pfs(&self) -> &P30pfs {
        self.p30pfs(2)
    }
    #[doc = "0xdc - Port 307 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p307pfs(&self) -> &P30pfs {
        self.p30pfs(3)
    }
    #[doc = "0xd2..0xda - Port 30%s Pin Function Select Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `P304PFS_HA` register.</div>"]
    #[inline(always)]
    pub const fn p30pfs_ha(&self, n: usize) -> &P30pfsHa {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(210)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd2..0xda - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p30pfs_ha_iter(&self) -> impl Iterator<Item = &P30pfsHa> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(210)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0xd2 - Port 304 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p304pfs_ha(&self) -> &P30pfsHa {
        self.p30pfs_ha(0)
    }
    #[doc = "0xd6 - Port 305 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p305pfs_ha(&self) -> &P30pfsHa {
        self.p30pfs_ha(1)
    }
    #[doc = "0xda - Port 306 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p306pfs_ha(&self) -> &P30pfsHa {
        self.p30pfs_ha(2)
    }
    #[doc = "0xde - Port 307 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p307pfs_ha(&self) -> &P30pfsHa {
        self.p30pfs_ha(3)
    }
    #[doc = "0xd3 - Port 30%s Pin Function Select Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `P304PFS_BY` register.</div>"]
    #[inline(always)]
    pub const fn p30pfs_by(&self, n: usize) -> &P30pfsBy {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(211)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd3 - Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p30pfs_by_iter(&self) -> impl Iterator<Item = &P30pfsBy> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(211)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0xd3 - Port 304 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p304pfs_by(&self) -> &P30pfsBy {
        self.p30pfs_by(0)
    }
    #[doc = "0xd7 - Port 305 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p305pfs_by(&self) -> &P30pfsBy {
        self.p30pfs_by(1)
    }
    #[doc = "0xdb - Port 306 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p306pfs_by(&self) -> &P30pfsBy {
        self.p30pfs_by(2)
    }
    #[doc = "0xdf - Port 307 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p307pfs_by(&self) -> &P30pfsBy {
        self.p30pfs_by(3)
    }
    #[doc = "0x100..0x110 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs(&self, n: usize) -> &P40pfs {
        &self.p40pfs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p40pfs_iter(&self) -> impl Iterator<Item = &P40pfs> {
        self.p40pfs.iter()
    }
    #[doc = "0x100 - Port 400 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p400pfs(&self) -> &P40pfs {
        self.p40pfs(0)
    }
    #[doc = "0x104 - Port 401 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p401pfs(&self) -> &P40pfs {
        self.p40pfs(1)
    }
    #[doc = "0x108 - Port 402 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p402pfs(&self) -> &P40pfs {
        self.p40pfs(2)
    }
    #[doc = "0x10c - Port 403 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p403pfs(&self) -> &P40pfs {
        self.p40pfs(3)
    }
    #[doc = "0x102..0x10a - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs_ha(&self, n: usize) -> &P40pfsHa {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(258)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x102..0x10a - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p40pfs_ha_iter(&self) -> impl Iterator<Item = &P40pfsHa> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(258)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x102 - Port 400 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p400pfs_ha(&self) -> &P40pfsHa {
        self.p40pfs_ha(0)
    }
    #[doc = "0x106 - Port 401 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p401pfs_ha(&self) -> &P40pfsHa {
        self.p40pfs_ha(1)
    }
    #[doc = "0x10a - Port 402 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p402pfs_ha(&self) -> &P40pfsHa {
        self.p40pfs_ha(2)
    }
    #[doc = "0x10e - Port 403 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p403pfs_ha(&self) -> &P40pfsHa {
        self.p40pfs_ha(3)
    }
    #[doc = "0x103 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs_by(&self, n: usize) -> &P40pfsBy {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(259)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x103 - Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub fn p40pfs_by_iter(&self) -> impl Iterator<Item = &P40pfsBy> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(259)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x103 - Port 400 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p400pfs_by(&self) -> &P40pfsBy {
        self.p40pfs_by(0)
    }
    #[doc = "0x107 - Port 401 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p401pfs_by(&self) -> &P40pfsBy {
        self.p40pfs_by(1)
    }
    #[doc = "0x10b - Port 402 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p402pfs_by(&self) -> &P40pfsBy {
        self.p40pfs_by(2)
    }
    #[doc = "0x10f - Port 403 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p403pfs_by(&self) -> &P40pfsBy {
        self.p40pfs_by(3)
    }
    #[doc = "0x503 - Write-Protect Register"]
    #[inline(always)]
    pub const fn pwpr(&self) -> &Pwpr {
        &self.pwpr
    }
    #[doc = "0x50f - Port Read Wait Control Register"]
    #[inline(always)]
    pub const fn prwcntr(&self) -> &Prwcntr {
        &self.prwcntr
    }
}
#[doc = "P00PFS (rw) register accessor: Port 00%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p00pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p00pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p00pfs`]
module"]
#[doc(alias = "P00PFS")]
pub type P00pfs = crate::Reg<p00pfs::P00pfsSpec>;
#[doc = "Port 00%s Pin Function Select Register"]
pub mod p00pfs;
#[doc = "P00PFS_HA (rw) register accessor: Port 00%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p00pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p00pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p00pfs_ha`]
module"]
#[doc(alias = "P00PFS_HA")]
pub type P00pfsHa = crate::Reg<p00pfs_ha::P00pfsHaSpec>;
#[doc = "Port 00%s Pin Function Select Register"]
pub mod p00pfs_ha;
#[doc = "P00PFS_BY (rw) register accessor: Port 00%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p00pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p00pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p00pfs_by`]
module"]
#[doc(alias = "P00PFS_BY")]
pub type P00pfsBy = crate::Reg<p00pfs_by::P00pfsBySpec>;
#[doc = "Port 00%s Pin Function Select Register"]
pub mod p00pfs_by;
#[doc = "P0PFS (rw) register accessor: Port 0%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p0pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0pfs`]
module"]
#[doc(alias = "P0PFS")]
pub type P0pfs = crate::Reg<p0pfs::P0pfsSpec>;
#[doc = "Port 0%s Pin Function Select Register"]
pub mod p0pfs;
#[doc = "P0PFS_HA (rw) register accessor: Port 0%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p0pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0pfs_ha`]
module"]
#[doc(alias = "P0PFS_HA")]
pub type P0pfsHa = crate::Reg<p0pfs_ha::P0pfsHaSpec>;
#[doc = "Port 0%s Pin Function Select Register"]
pub mod p0pfs_ha;
#[doc = "P0PFS_BY (rw) register accessor: Port 0%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p0pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0pfs_by`]
module"]
#[doc(alias = "P0PFS_BY")]
pub type P0pfsBy = crate::Reg<p0pfs_by::P0pfsBySpec>;
#[doc = "Port 0%s Pin Function Select Register"]
pub mod p0pfs_by;
#[doc = "P10PFS (rw) register accessor: Port 10%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p10pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p10pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p10pfs`]
module"]
#[doc(alias = "P10PFS")]
pub type P10pfs = crate::Reg<p10pfs::P10pfsSpec>;
#[doc = "Port 10%s Pin Function Select Register"]
pub mod p10pfs;
#[doc = "P10PFS_HA (rw) register accessor: Port 10%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p10pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p10pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p10pfs_ha`]
module"]
#[doc(alias = "P10PFS_HA")]
pub type P10pfsHa = crate::Reg<p10pfs_ha::P10pfsHaSpec>;
#[doc = "Port 10%s Pin Function Select Register"]
pub mod p10pfs_ha;
#[doc = "P10PFS_BY (rw) register accessor: Port 10%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p10pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p10pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p10pfs_by`]
module"]
#[doc(alias = "P10PFS_BY")]
pub type P10pfsBy = crate::Reg<p10pfs_by::P10pfsBySpec>;
#[doc = "Port 10%s Pin Function Select Register"]
pub mod p10pfs_by;
#[doc = "P1PFS (rw) register accessor: Port 1%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1pfs`]
module"]
#[doc(alias = "P1PFS")]
pub type P1pfs = crate::Reg<p1pfs::P1pfsSpec>;
#[doc = "Port 1%s Pin Function Select Register"]
pub mod p1pfs;
#[doc = "P1PFS_HA (rw) register accessor: Port 1%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1pfs_ha`]
module"]
#[doc(alias = "P1PFS_HA")]
pub type P1pfsHa = crate::Reg<p1pfs_ha::P1pfsHaSpec>;
#[doc = "Port 1%s Pin Function Select Register"]
pub mod p1pfs_ha;
#[doc = "P1PFS_BY (rw) register accessor: Port 1%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1pfs_by`]
module"]
#[doc(alias = "P1PFS_BY")]
pub type P1pfsBy = crate::Reg<p1pfs_by::P1pfsBySpec>;
#[doc = "Port 1%s Pin Function Select Register"]
pub mod p1pfs_by;
#[doc = "P20PFS (rw) register accessor: Port 20%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p20pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p20pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p20pfs`]
module"]
#[doc(alias = "P20PFS")]
pub type P20pfs = crate::Reg<p20pfs::P20pfsSpec>;
#[doc = "Port 20%s Pin Function Select Register"]
pub mod p20pfs;
#[doc = "P20PFS_HA (rw) register accessor: Port 20%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p20pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p20pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p20pfs_ha`]
module"]
#[doc(alias = "P20PFS_HA")]
pub type P20pfsHa = crate::Reg<p20pfs_ha::P20pfsHaSpec>;
#[doc = "Port 20%s Pin Function Select Register"]
pub mod p20pfs_ha;
#[doc = "P20PFS_BY (rw) register accessor: Port 20%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p20pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p20pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p20pfs_by`]
module"]
#[doc(alias = "P20PFS_BY")]
pub type P20pfsBy = crate::Reg<p20pfs_by::P20pfsBySpec>;
#[doc = "Port 20%s Pin Function Select Register"]
pub mod p20pfs_by;
#[doc = "P203PFS (rw) register accessor: Port 203 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p203pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p203pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p203pfs`]
module"]
#[doc(alias = "P203PFS")]
pub type P203pfs = crate::Reg<p203pfs::P203pfsSpec>;
#[doc = "Port 203 Pin Function Select Register"]
pub mod p203pfs;
#[doc = "P203PFS_HA (rw) register accessor: Port 203 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p203pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p203pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p203pfs_ha`]
module"]
#[doc(alias = "P203PFS_HA")]
pub type P203pfsHa = crate::Reg<p203pfs_ha::P203pfsHaSpec>;
#[doc = "Port 203 Pin Function Select Register"]
pub mod p203pfs_ha;
#[doc = "P203PFS_BY (rw) register accessor: Port 203 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p203pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p203pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p203pfs_by`]
module"]
#[doc(alias = "P203PFS_BY")]
pub type P203pfsBy = crate::Reg<p203pfs_by::P203pfsBySpec>;
#[doc = "Port 203 Pin Function Select Register"]
pub mod p203pfs_by;
pub use p20pfs as p204pfs;
pub use p20pfs_by as p204pfs_by;
pub use p20pfs_ha as p204pfs_ha;
pub use P20pfs as P204pfs;
pub use P20pfsBy as P204pfsBy;
pub use P20pfsHa as P204pfsHa;
#[doc = "P300PFS (rw) register accessor: Port 300 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p300pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p300pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p300pfs`]
module"]
#[doc(alias = "P300PFS")]
pub type P300pfs = crate::Reg<p300pfs::P300pfsSpec>;
#[doc = "Port 300 Pin Function Select Register"]
pub mod p300pfs;
#[doc = "P300PFS_HA (rw) register accessor: Port 300 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p300pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p300pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p300pfs_ha`]
module"]
#[doc(alias = "P300PFS_HA")]
pub type P300pfsHa = crate::Reg<p300pfs_ha::P300pfsHaSpec>;
#[doc = "Port 300 Pin Function Select Register"]
pub mod p300pfs_ha;
#[doc = "P300PFS_BY (rw) register accessor: Port 300 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p300pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p300pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p300pfs_by`]
module"]
#[doc(alias = "P300PFS_BY")]
pub type P300pfsBy = crate::Reg<p300pfs_by::P300pfsBySpec>;
#[doc = "Port 300 Pin Function Select Register"]
pub mod p300pfs_by;
#[doc = "P301PFS (rw) register accessor: Port 301 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p301pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p301pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p301pfs`]
module"]
#[doc(alias = "P301PFS")]
pub type P301pfs = crate::Reg<p301pfs::P301pfsSpec>;
#[doc = "Port 301 Pin Function Select Register"]
pub mod p301pfs;
#[doc = "P301PFS_HA (rw) register accessor: Port 301 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p301pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p301pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p301pfs_ha`]
module"]
#[doc(alias = "P301PFS_HA")]
pub type P301pfsHa = crate::Reg<p301pfs_ha::P301pfsHaSpec>;
#[doc = "Port 301 Pin Function Select Register"]
pub mod p301pfs_ha;
#[doc = "P301PFS_BY (rw) register accessor: Port 301 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p301pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p301pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p301pfs_by`]
module"]
#[doc(alias = "P301PFS_BY")]
pub type P301pfsBy = crate::Reg<p301pfs_by::P301pfsBySpec>;
#[doc = "Port 301 Pin Function Select Register"]
pub mod p301pfs_by;
#[doc = "P302PFS (rw) register accessor: Port 302 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p302pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p302pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p302pfs`]
module"]
#[doc(alias = "P302PFS")]
pub type P302pfs = crate::Reg<p302pfs::P302pfsSpec>;
#[doc = "Port 302 Pin Function Select Register"]
pub mod p302pfs;
pub use p30pfs_by as p302pfs_by;
pub use p30pfs_ha as p302pfs_ha;
pub use P30pfsBy as P302pfsBy;
pub use P30pfsHa as P302pfsHa;
#[doc = "P303PFS (rw) register accessor: Port 303 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p303pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p303pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p303pfs`]
module"]
#[doc(alias = "P303PFS")]
pub type P303pfs = crate::Reg<p303pfs::P303pfsSpec>;
#[doc = "Port 303 Pin Function Select Register"]
pub mod p303pfs;
#[doc = "P303PFS_HA (rw) register accessor: Port 303 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p303pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p303pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p303pfs_ha`]
module"]
#[doc(alias = "P303PFS_HA")]
pub type P303pfsHa = crate::Reg<p303pfs_ha::P303pfsHaSpec>;
#[doc = "Port 303 Pin Function Select Register"]
pub mod p303pfs_ha;
#[doc = "P303PFS_BY (rw) register accessor: Port 303 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p303pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p303pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p303pfs_by`]
module"]
#[doc(alias = "P303PFS_BY")]
pub type P303pfsBy = crate::Reg<p303pfs_by::P303pfsBySpec>;
#[doc = "Port 303 Pin Function Select Register"]
pub mod p303pfs_by;
#[doc = "P30PFS (rw) register accessor: Port 30%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p30pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p30pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p30pfs`]
module"]
#[doc(alias = "P30PFS")]
pub type P30pfs = crate::Reg<p30pfs::P30pfsSpec>;
#[doc = "Port 30%s Pin Function Select Register"]
pub mod p30pfs;
#[doc = "P30PFS_HA (rw) register accessor: Port 30%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p30pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p30pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p30pfs_ha`]
module"]
#[doc(alias = "P30PFS_HA")]
pub type P30pfsHa = crate::Reg<p30pfs_ha::P30pfsHaSpec>;
#[doc = "Port 30%s Pin Function Select Register"]
pub mod p30pfs_ha;
#[doc = "P30PFS_BY (rw) register accessor: Port 30%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p30pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p30pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p30pfs_by`]
module"]
#[doc(alias = "P30PFS_BY")]
pub type P30pfsBy = crate::Reg<p30pfs_by::P30pfsBySpec>;
#[doc = "Port 30%s Pin Function Select Register"]
pub mod p30pfs_by;
#[doc = "P40PFS (rw) register accessor: Port 40%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p40pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p40pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p40pfs`]
module"]
#[doc(alias = "P40PFS")]
pub type P40pfs = crate::Reg<p40pfs::P40pfsSpec>;
#[doc = "Port 40%s Pin Function Select Register"]
pub mod p40pfs;
#[doc = "P40PFS_HA (rw) register accessor: Port 40%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p40pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p40pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p40pfs_ha`]
module"]
#[doc(alias = "P40PFS_HA")]
pub type P40pfsHa = crate::Reg<p40pfs_ha::P40pfsHaSpec>;
#[doc = "Port 40%s Pin Function Select Register"]
pub mod p40pfs_ha;
#[doc = "P40PFS_BY (rw) register accessor: Port 40%s Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p40pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p40pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p40pfs_by`]
module"]
#[doc(alias = "P40PFS_BY")]
pub type P40pfsBy = crate::Reg<p40pfs_by::P40pfsBySpec>;
#[doc = "Port 40%s Pin Function Select Register"]
pub mod p40pfs_by;
#[doc = "PWPR (rw) register accessor: Write-Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwpr`]
module"]
#[doc(alias = "PWPR")]
pub type Pwpr = crate::Reg<pwpr::PwprSpec>;
#[doc = "Write-Protect Register"]
pub mod pwpr;
#[doc = "PRWCNTR (w) register accessor: Port Read Wait Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prwcntr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prwcntr`]
module"]
#[doc(alias = "PRWCNTR")]
pub type Prwcntr = crate::Reg<prwcntr::PrwcntrSpec>;
#[doc = "Port Read Wait Control Register"]
pub mod prwcntr;
