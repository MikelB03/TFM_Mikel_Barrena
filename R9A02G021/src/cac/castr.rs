#[doc = "Register `CASTR` reader"]
pub type R = crate::R<CastrSpec>;
#[doc = "Frequency Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ferrf {
    #[doc = "0: Clock frequency is within the allowable range"]
    _0 = 0,
    #[doc = "1: Clock frequency has deviated beyond the allowable range (frequency error)."]
    _1 = 1,
}
impl From<Ferrf> for bool {
    #[inline(always)]
    fn from(variant: Ferrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FERRF` reader - Frequency Error Flag"]
pub type FerrfR = crate::BitReader<Ferrf>;
impl FerrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ferrf {
        match self.bits {
            false => Ferrf::_0,
            true => Ferrf::_1,
        }
    }
    #[doc = "Clock frequency is within the allowable range"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ferrf::_0
    }
    #[doc = "Clock frequency has deviated beyond the allowable range (frequency error)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ferrf::_1
    }
}
#[doc = "Measurement End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mendf {
    #[doc = "0: Measurement is in progress"]
    _0 = 0,
    #[doc = "1: Measurement ended"]
    _1 = 1,
}
impl From<Mendf> for bool {
    #[inline(always)]
    fn from(variant: Mendf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MENDF` reader - Measurement End Flag"]
pub type MendfR = crate::BitReader<Mendf>;
impl MendfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mendf {
        match self.bits {
            false => Mendf::_0,
            true => Mendf::_1,
        }
    }
    #[doc = "Measurement is in progress"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mendf::_0
    }
    #[doc = "Measurement ended"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mendf::_1
    }
}
#[doc = "Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovff {
    #[doc = "0: Counter has not overflowed"]
    _0 = 0,
    #[doc = "1: Counter overflowed"]
    _1 = 1,
}
impl From<Ovff> for bool {
    #[inline(always)]
    fn from(variant: Ovff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFF` reader - Overflow Flag"]
pub type OvffR = crate::BitReader<Ovff>;
impl OvffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovff {
        match self.bits {
            false => Ovff::_0,
            true => Ovff::_1,
        }
    }
    #[doc = "Counter has not overflowed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovff::_0
    }
    #[doc = "Counter overflowed"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovff::_1
    }
}
impl R {
    #[doc = "Bit 0 - Frequency Error Flag"]
    #[inline(always)]
    pub fn ferrf(&self) -> FerrfR {
        FerrfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Measurement End Flag"]
    #[inline(always)]
    pub fn mendf(&self) -> MendfR {
        MendfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn ovff(&self) -> OvffR {
        OvffR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "CAC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`castr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CastrSpec;
impl crate::RegisterSpec for CastrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`castr::R`](R) reader structure"]
impl crate::Readable for CastrSpec {}
#[doc = "`reset()` method sets CASTR to value 0"]
impl crate::Resettable for CastrSpec {
    const RESET_VALUE: u8 = 0;
}
