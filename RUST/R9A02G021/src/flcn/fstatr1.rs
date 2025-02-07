#[doc = "Register `FSTATR1` reader"]
pub type R = crate::R<Fstatr1Spec>;
#[doc = "Flash Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frdy {
    #[doc = "0: The software command of the FCR register is not terminated."]
    _0 = 0,
    #[doc = "1: The software command of the FCR register is terminated."]
    _1 = 1,
}
impl From<Frdy> for bool {
    #[inline(always)]
    fn from(variant: Frdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRDY` reader - Flash Ready Flag"]
pub type FrdyR = crate::BitReader<Frdy>;
impl FrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frdy {
        match self.bits {
            false => Frdy::_0,
            true => Frdy::_1,
        }
    }
    #[doc = "The software command of the FCR register is not terminated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Frdy::_0
    }
    #[doc = "The software command of the FCR register is terminated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Frdy::_1
    }
}
#[doc = "Extra Area Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exrdy {
    #[doc = "0: The software command of the FEXCR register is not terminated."]
    _0 = 0,
    #[doc = "1: The software command of the FEXCR register is terminated."]
    _1 = 1,
}
impl From<Exrdy> for bool {
    #[inline(always)]
    fn from(variant: Exrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXRDY` reader - Extra Area Ready Flag"]
pub type ExrdyR = crate::BitReader<Exrdy>;
impl ExrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exrdy {
        match self.bits {
            false => Exrdy::_0,
            true => Exrdy::_1,
        }
    }
    #[doc = "The software command of the FEXCR register is not terminated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Exrdy::_0
    }
    #[doc = "The software command of the FEXCR register is terminated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Exrdy::_1
    }
}
impl R {
    #[doc = "Bit 6 - Flash Ready Flag"]
    #[inline(always)]
    pub fn frdy(&self) -> FrdyR {
        FrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Extra Area Ready Flag"]
    #[inline(always)]
    pub fn exrdy(&self) -> ExrdyR {
        ExrdyR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Flash Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fstatr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fstatr1Spec;
impl crate::RegisterSpec for Fstatr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fstatr1::R`](R) reader structure"]
impl crate::Readable for Fstatr1Spec {}
#[doc = "`reset()` method sets FSTATR1 to value 0x04"]
impl crate::Resettable for Fstatr1Spec {
    const RESET_VALUE: u8 = 0x04;
}
