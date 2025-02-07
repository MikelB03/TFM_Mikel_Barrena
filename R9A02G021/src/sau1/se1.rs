#[doc = "Register `SE1` reader"]
pub type R = crate::R<Se1Spec>;
#[doc = "Indication of whether operation of channel n is enabled or stopped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Se {
    #[doc = "0: Operation stops"]
    _0 = 0,
    #[doc = "1: Operation is enabled"]
    _1 = 1,
}
impl From<Se> for u8 {
    #[inline(always)]
    fn from(variant: Se) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Se {
    type Ux = u8;
}
impl crate::IsEnum for Se {}
#[doc = "Field `SE` reader - Indication of whether operation of channel n is enabled or stopped."]
pub type SeR = crate::FieldReader<Se>;
impl SeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Se> {
        match self.bits {
            0 => Some(Se::_0),
            1 => Some(Se::_1),
            _ => None,
        }
    }
    #[doc = "Operation stops"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Se::_0
    }
    #[doc = "Operation is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Se::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Indication of whether operation of channel n is enabled or stopped."]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new((self.bits & 3) as u8)
    }
}
#[doc = "Serial Channel Enable Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`se1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Se1Spec;
impl crate::RegisterSpec for Se1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`se1::R`](R) reader structure"]
impl crate::Readable for Se1Spec {}
#[doc = "`reset()` method sets SE1 to value 0"]
impl crate::Resettable for Se1Spec {
    const RESET_VALUE: u16 = 0;
}
