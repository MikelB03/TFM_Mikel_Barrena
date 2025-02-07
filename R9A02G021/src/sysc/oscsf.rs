#[doc = "Register `OSCSF` reader"]
pub type R = crate::R<OscsfSpec>;
#[doc = "HOCO Clock Oscillation Stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hocosf {
    #[doc = "0: The HOCO clock is stopped or is not yet stable"]
    _0 = 0,
    #[doc = "1: The HOCO clock is stable, so is available for use as the system clock"]
    _1 = 1,
}
impl From<Hocosf> for bool {
    #[inline(always)]
    fn from(variant: Hocosf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOCOSF` reader - HOCO Clock Oscillation Stabilization Flag"]
pub type HocosfR = crate::BitReader<Hocosf>;
impl HocosfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hocosf {
        match self.bits {
            false => Hocosf::_0,
            true => Hocosf::_1,
        }
    }
    #[doc = "The HOCO clock is stopped or is not yet stable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hocosf::_0
    }
    #[doc = "The HOCO clock is stable, so is available for use as the system clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hocosf::_1
    }
}
impl R {
    #[doc = "Bit 0 - HOCO Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn hocosf(&self) -> HocosfR {
        HocosfR::new((self.bits & 1) != 0)
    }
}
#[doc = "Oscillation Stabilization Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oscsf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscsfSpec;
impl crate::RegisterSpec for OscsfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oscsf::R`](R) reader structure"]
impl crate::Readable for OscsfSpec {}
#[doc = "`reset()` method sets OSCSF to value 0"]
impl crate::Resettable for OscsfSpec {
    const RESET_VALUE: u8 = 0;
}
