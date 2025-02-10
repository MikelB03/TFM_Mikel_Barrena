#[doc = "Register `TE0` reader"]
pub type R = crate::R<Te0Spec>;
#[doc = "Indication of operation enabled or disabled state of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Te {
    #[doc = "0: Operation is disabled"]
    _0 = 0,
    #[doc = "1: Operation is enabled"]
    _1 = 1,
}
impl From<Te> for u8 {
    #[inline(always)]
    fn from(variant: Te) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Te {
    type Ux = u8;
}
impl crate::IsEnum for Te {}
#[doc = "Field `TE` reader - Indication of operation enabled or disabled state of channel n"]
pub type TeR = crate::FieldReader<Te>;
impl TeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Te> {
        match self.bits {
            0 => Some(Te::_0),
            1 => Some(Te::_1),
            _ => None,
        }
    }
    #[doc = "Operation is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Te::_0
    }
    #[doc = "Operation is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Te::_1
    }
}
#[doc = "Indication of whether operation of the higher 8-bit timer is enabled or disabled when channel 1 is in the 8-bit timer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teh1 {
    #[doc = "0: Operation is disabled"]
    _0 = 0,
    #[doc = "1: Operation is enabled"]
    _1 = 1,
}
impl From<Teh1> for bool {
    #[inline(always)]
    fn from(variant: Teh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEH1` reader - Indication of whether operation of the higher 8-bit timer is enabled or disabled when channel 1 is in the 8-bit timer mode"]
pub type Teh1R = crate::BitReader<Teh1>;
impl Teh1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teh1 {
        match self.bits {
            false => Teh1::_0,
            true => Teh1::_1,
        }
    }
    #[doc = "Operation is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Teh1::_0
    }
    #[doc = "Operation is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Teh1::_1
    }
}
#[doc = "Indication of whether operation of the higher 8-bit timer is enabled or disabled when channel 3 is in the 8-bit timer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teh3 {
    #[doc = "0: Operation is disabled"]
    _0 = 0,
    #[doc = "1: Operation is enabled"]
    _1 = 1,
}
impl From<Teh3> for bool {
    #[inline(always)]
    fn from(variant: Teh3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEH3` reader - Indication of whether operation of the higher 8-bit timer is enabled or disabled when channel 3 is in the 8-bit timer mode"]
pub type Teh3R = crate::BitReader<Teh3>;
impl Teh3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teh3 {
        match self.bits {
            false => Teh3::_0,
            true => Teh3::_1,
        }
    }
    #[doc = "Operation is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Teh3::_0
    }
    #[doc = "Operation is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Teh3::_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Indication of operation enabled or disabled state of channel n"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - Indication of whether operation of the higher 8-bit timer is enabled or disabled when channel 1 is in the 8-bit timer mode"]
    #[inline(always)]
    pub fn teh1(&self) -> Teh1R {
        Teh1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Indication of whether operation of the higher 8-bit timer is enabled or disabled when channel 3 is in the 8-bit timer mode"]
    #[inline(always)]
    pub fn teh3(&self) -> Teh3R {
        Teh3R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Timer Channel Enable Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`te0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Te0Spec;
impl crate::RegisterSpec for Te0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`te0::R`](R) reader structure"]
impl crate::Readable for Te0Spec {}
#[doc = "`reset()` method sets TE0 to value 0"]
impl crate::Resettable for Te0Spec {
    const RESET_VALUE: u16 = 0;
}
