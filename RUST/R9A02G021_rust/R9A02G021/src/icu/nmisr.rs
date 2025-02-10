#[doc = "Register `NMISR` reader"]
pub type R = crate::R<NmisrSpec>;
#[doc = "IWDT Underflow/Refresh Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtst {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<Iwdtst> for bool {
    #[inline(always)]
    fn from(variant: Iwdtst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDTST` reader - IWDT Underflow/Refresh Error Interrupt Status Flag"]
pub type IwdtstR = crate::BitReader<Iwdtst>;
impl IwdtstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtst {
        match self.bits {
            false => Iwdtst::_0,
            true => Iwdtst::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtst::_0
    }
    #[doc = "Interrupt requested"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtst::_1
    }
}
#[doc = "WDT Underflow/Refresh Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtst {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<Wdtst> for bool {
    #[inline(always)]
    fn from(variant: Wdtst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTST` reader - WDT Underflow/Refresh Error Interrupt Status Flag"]
pub type WdtstR = crate::BitReader<Wdtst>;
impl WdtstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtst {
        match self.bits {
            false => Wdtst::_0,
            true => Wdtst::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdtst::_0
    }
    #[doc = "Interrupt requested"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdtst::_1
    }
}
#[doc = "Voltage Monitor 1 Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1st {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<Lvd1st> for bool {
    #[inline(always)]
    fn from(variant: Lvd1st) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD1ST` reader - Voltage Monitor 1 Interrupt Status Flag"]
pub type Lvd1stR = crate::BitReader<Lvd1st>;
impl Lvd1stR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1st {
        match self.bits {
            false => Lvd1st::_0,
            true => Lvd1st::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1st::_0
    }
    #[doc = "Interrupt requested"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1st::_1
    }
}
#[doc = "Voltage Monitor 2 Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2st {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<Lvd2st> for bool {
    #[inline(always)]
    fn from(variant: Lvd2st) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD2ST` reader - Voltage Monitor 2 Interrupt Status Flag"]
pub type Lvd2stR = crate::BitReader<Lvd2st>;
impl Lvd2stR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2st {
        match self.bits {
            false => Lvd2st::_0,
            true => Lvd2st::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2st::_0
    }
    #[doc = "Interrupt requested"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2st::_1
    }
}
#[doc = "NMI Pin Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmist {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<Nmist> for bool {
    #[inline(always)]
    fn from(variant: Nmist) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIST` reader - NMI Pin Interrupt Status Flag"]
pub type NmistR = crate::BitReader<Nmist>;
impl NmistR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmist {
        match self.bits {
            false => Nmist::_0,
            true => Nmist::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nmist::_0
    }
    #[doc = "Interrupt requested"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nmist::_1
    }
}
#[doc = "SRAM Parity Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpest {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<Rpest> for bool {
    #[inline(always)]
    fn from(variant: Rpest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPEST` reader - SRAM Parity Error Interrupt Status Flag"]
pub type RpestR = crate::BitReader<Rpest>;
impl RpestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpest {
        match self.bits {
            false => Rpest::_0,
            true => Rpest::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpest::_0
    }
    #[doc = "Interrupt requested"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpest::_1
    }
}
#[doc = "SRAM ECC Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reccst {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<Reccst> for bool {
    #[inline(always)]
    fn from(variant: Reccst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECCST` reader - SRAM ECC Error Interrupt Status Flag"]
pub type ReccstR = crate::BitReader<Reccst>;
impl ReccstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reccst {
        match self.bits {
            false => Reccst::_0,
            true => Reccst::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reccst::_0
    }
    #[doc = "Interrupt requested"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reccst::_1
    }
}
#[doc = "BUS Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busst {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<Busst> for bool {
    #[inline(always)]
    fn from(variant: Busst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSST` reader - BUS Error Interrupt Status Flag"]
pub type BusstR = crate::BitReader<Busst>;
impl BusstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busst {
        match self.bits {
            false => Busst::_0,
            true => Busst::_1,
        }
    }
    #[doc = "Interrupt not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busst::_0
    }
    #[doc = "Interrupt requested"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busst::_1
    }
}
impl R {
    #[doc = "Bit 0 - IWDT Underflow/Refresh Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn iwdtst(&self) -> IwdtstR {
        IwdtstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT Underflow/Refresh Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn wdtst(&self) -> WdtstR {
        WdtstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Interrupt Status Flag"]
    #[inline(always)]
    pub fn lvd1st(&self) -> Lvd1stR {
        Lvd1stR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Monitor 2 Interrupt Status Flag"]
    #[inline(always)]
    pub fn lvd2st(&self) -> Lvd2stR {
        Lvd2stR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - NMI Pin Interrupt Status Flag"]
    #[inline(always)]
    pub fn nmist(&self) -> NmistR {
        NmistR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM Parity Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn rpest(&self) -> RpestR {
        RpestR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM ECC Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn reccst(&self) -> ReccstR {
        ReccstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - BUS Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn busst(&self) -> BusstR {
        BusstR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Non-Maskable Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmisrSpec;
impl crate::RegisterSpec for NmisrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nmisr::R`](R) reader structure"]
impl crate::Readable for NmisrSpec {}
#[doc = "`reset()` method sets NMISR to value 0"]
impl crate::Resettable for NmisrSpec {
    const RESET_VALUE: u16 = 0;
}
