#[doc = "Register `TSR0%s` reader"]
pub type R = crate::R<Tsr0Spec>;
#[doc = "Counter overflow state of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovf {
    #[doc = "0: Overflow does not occur"]
    _0 = 0,
    #[doc = "1: Overflow occurs"]
    _1 = 1,
}
impl From<Ovf> for bool {
    #[inline(always)]
    fn from(variant: Ovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVF` reader - Counter overflow state of channel n"]
pub type OvfR = crate::BitReader<Ovf>;
impl OvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovf {
        match self.bits {
            false => Ovf::_0,
            true => Ovf::_1,
        }
    }
    #[doc = "Overflow does not occur"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovf::_0
    }
    #[doc = "Overflow occurs"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovf::_1
    }
}
impl R {
    #[doc = "Bit 0 - Counter overflow state of channel n"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
}
#[doc = "Timer Status Register 0%s\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tsr0Spec;
impl crate::RegisterSpec for Tsr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tsr0::R`](R) reader structure"]
impl crate::Readable for Tsr0Spec {}
#[doc = "`reset()` method sets TSR0%s to value 0"]
impl crate::Resettable for Tsr0Spec {
    const RESET_VALUE: u16 = 0;
}
