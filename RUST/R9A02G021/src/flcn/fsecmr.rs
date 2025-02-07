#[doc = "Register `FSECMR` reader"]
pub type R = crate::R<FsecmrSpec>;
#[doc = "Startup Area Select Protection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Btflg {
    #[doc = "0: Startup area is the alternate block (block 1)"]
    _0 = 0,
    #[doc = "1: Startup area is the alternate block (block 0)"]
    _1 = 1,
}
impl From<Btflg> for bool {
    #[inline(always)]
    fn from(variant: Btflg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTFLG` reader - Startup Area Select Protection Flag"]
pub type BtflgR = crate::BitReader<Btflg>;
impl BtflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Btflg {
        match self.bits {
            false => Btflg::_0,
            true => Btflg::_1,
        }
    }
    #[doc = "Startup area is the alternate block (block 1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Btflg::_0
    }
    #[doc = "Startup area is the alternate block (block 0)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Btflg::_1
    }
}
#[doc = "Startup Area Select Protection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Btpr {
    #[doc = "0: Startup area select setting is locked"]
    _0 = 0,
    #[doc = "1: Startup area select setting is not locked"]
    _1 = 1,
}
impl From<Btpr> for bool {
    #[inline(always)]
    fn from(variant: Btpr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTPR` reader - Startup Area Select Protection Flag"]
pub type BtprR = crate::BitReader<Btpr>;
impl BtprR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Btpr {
        match self.bits {
            false => Btpr::_0,
            true => Btpr::_1,
        }
    }
    #[doc = "Startup area select setting is locked"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Btpr::_0
    }
    #[doc = "Startup area select setting is not locked"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Btpr::_1
    }
}
#[doc = "On-Chip Debugger Connection Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocddis {
    #[doc = "0: On-chip debugger connection disabled"]
    _0 = 0,
    #[doc = "1: On-chip debugger connection enabled"]
    _1 = 1,
}
impl From<Ocddis> for bool {
    #[inline(always)]
    fn from(variant: Ocddis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCDDIS` reader - On-Chip Debugger Connection Disable Flag"]
pub type OcddisR = crate::BitReader<Ocddis>;
impl OcddisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ocddis {
        match self.bits {
            false => Ocddis::_0,
            true => Ocddis::_1,
        }
    }
    #[doc = "On-chip debugger connection disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ocddis::_0
    }
    #[doc = "On-chip debugger connection enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ocddis::_1
    }
}
#[doc = "Access Window Protection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fapr {
    #[doc = "0: Access window setting is locked"]
    _0 = 0,
    #[doc = "1: Access window setting is not locked"]
    _1 = 1,
}
impl From<Fapr> for bool {
    #[inline(always)]
    fn from(variant: Fapr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAPR` reader - Access Window Protection Flag"]
pub type FaprR = crate::BitReader<Fapr>;
impl FaprR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fapr {
        match self.bits {
            false => Fapr::_0,
            true => Fapr::_1,
        }
    }
    #[doc = "Access window setting is locked"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fapr::_0
    }
    #[doc = "Access window setting is not locked"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fapr::_1
    }
}
impl R {
    #[doc = "Bit 8 - Startup Area Select Protection Flag"]
    #[inline(always)]
    pub fn btflg(&self) -> BtflgR {
        BtflgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Startup Area Select Protection Flag"]
    #[inline(always)]
    pub fn btpr(&self) -> BtprR {
        BtprR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - On-Chip Debugger Connection Disable Flag"]
    #[inline(always)]
    pub fn ocddis(&self) -> OcddisR {
        OcddisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Access Window Protection Flag"]
    #[inline(always)]
    pub fn fapr(&self) -> FaprR {
        FaprR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Flash Protection Flag Monitor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsecmr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsecmrSpec;
impl crate::RegisterSpec for FsecmrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fsecmr::R`](R) reader structure"]
impl crate::Readable for FsecmrSpec {}
#[doc = "`reset()` method sets FSECMR to value 0"]
impl crate::Resettable for FsecmrSpec {
    const RESET_VALUE: u16 = 0;
}
