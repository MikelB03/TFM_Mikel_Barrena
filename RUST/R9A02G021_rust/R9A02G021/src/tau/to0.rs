#[doc = "Register `TO0` reader"]
pub type R = crate::R<To0Spec>;
#[doc = "Register `TO0` writer"]
pub type W = crate::W<To0Spec>;
#[doc = "Timer output of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum To {
    #[doc = "0: Timer output value is 0"]
    _0 = 0,
    #[doc = "1: Timer output value is 1"]
    _1 = 1,
}
impl From<To> for u8 {
    #[inline(always)]
    fn from(variant: To) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for To {
    type Ux = u8;
}
impl crate::IsEnum for To {}
#[doc = "Field `TO` reader - Timer output of channel n"]
pub type ToR = crate::FieldReader<To>;
impl ToR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<To> {
        match self.bits {
            0 => Some(To::_0),
            1 => Some(To::_1),
            _ => None,
        }
    }
    #[doc = "Timer output value is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == To::_0
    }
    #[doc = "Timer output value is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == To::_1
    }
}
#[doc = "Field `TO` writer - Timer output of channel n"]
pub type ToW<'a, REG> = crate::FieldWriter<'a, REG, 8, To>;
impl<'a, REG> ToW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer output value is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(To::_0)
    }
    #[doc = "Timer output value is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(To::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Timer output of channel n"]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer output of channel n"]
    #[inline(always)]
    pub fn to(&mut self) -> ToW<To0Spec> {
        ToW::new(self, 0)
    }
}
#[doc = "Timer Output Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`to0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct To0Spec;
impl crate::RegisterSpec for To0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`to0::R`](R) reader structure"]
impl crate::Readable for To0Spec {}
#[doc = "`write(|w| ..)` method takes [`to0::W`](W) writer structure"]
impl crate::Writable for To0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TO0 to value 0"]
impl crate::Resettable for To0Spec {
    const RESET_VALUE: u16 = 0;
}
