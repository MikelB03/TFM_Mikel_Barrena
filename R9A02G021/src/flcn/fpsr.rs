#[doc = "Register `FPSR` reader"]
pub type R = crate::R<FpsrSpec>;
#[doc = "Protect Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perr {
    #[doc = "0: No error"]
    _0 = 0,
    #[doc = "1: An error occurs"]
    _1 = 1,
}
impl From<Perr> for bool {
    #[inline(always)]
    fn from(variant: Perr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERR` reader - Protect Error Flag"]
pub type PerrR = crate::BitReader<Perr>;
impl PerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perr {
        match self.bits {
            false => Perr::_0,
            true => Perr::_1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Perr::_0
    }
    #[doc = "An error occurs"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Perr::_1
    }
}
impl R {
    #[doc = "Bit 0 - Protect Error Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Protection Unlock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpsrSpec;
impl crate::RegisterSpec for FpsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fpsr::R`](R) reader structure"]
impl crate::Readable for FpsrSpec {}
#[doc = "`reset()` method sets FPSR to value 0"]
impl crate::Resettable for FpsrSpec {
    const RESET_VALUE: u8 = 0;
}
