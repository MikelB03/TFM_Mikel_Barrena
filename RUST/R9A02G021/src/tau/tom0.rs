#[doc = "Register `TOM0` reader"]
pub type R = crate::R<Tom0Spec>;
#[doc = "Register `TOM0` writer"]
pub type W = crate::W<Tom0Spec>;
#[doc = "Control of timer output mode of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tom {
    #[doc = "0: Master channel output mode (to produce toggled output by timer interrupt request signal (TAU0_ENDIn))"]
    _0 = 0,
    #[doc = "1: Slave channel output mode (output is set by the timer interrupt request signal (TAU0_ENDIn) of the master channel, and reset by the timer interrupt request signal (TAU0_ENDIp) of the slave channel)"]
    _1 = 1,
}
impl From<Tom> for u8 {
    #[inline(always)]
    fn from(variant: Tom) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tom {
    type Ux = u8;
}
impl crate::IsEnum for Tom {}
#[doc = "Field `TOM` reader - Control of timer output mode of channel n"]
pub type TomR = crate::FieldReader<Tom>;
impl TomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tom> {
        match self.bits {
            0 => Some(Tom::_0),
            1 => Some(Tom::_1),
            _ => None,
        }
    }
    #[doc = "Master channel output mode (to produce toggled output by timer interrupt request signal (TAU0_ENDIn))"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tom::_0
    }
    #[doc = "Slave channel output mode (output is set by the timer interrupt request signal (TAU0_ENDIn) of the master channel, and reset by the timer interrupt request signal (TAU0_ENDIp) of the slave channel)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tom::_1
    }
}
#[doc = "Field `TOM` writer - Control of timer output mode of channel n"]
pub type TomW<'a, REG> = crate::FieldWriter<'a, REG, 7, Tom>;
impl<'a, REG> TomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master channel output mode (to produce toggled output by timer interrupt request signal (TAU0_ENDIn))"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tom::_0)
    }
    #[doc = "Slave channel output mode (output is set by the timer interrupt request signal (TAU0_ENDIn) of the master channel, and reset by the timer interrupt request signal (TAU0_ENDIp) of the slave channel)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tom::_1)
    }
}
impl R {
    #[doc = "Bits 1:7 - Control of timer output mode of channel n"]
    #[inline(always)]
    pub fn tom(&self) -> TomR {
        TomR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Control of timer output mode of channel n"]
    #[inline(always)]
    pub fn tom(&mut self) -> TomW<Tom0Spec> {
        TomW::new(self, 1)
    }
}
#[doc = "Timer Output Mode Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tom0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tom0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tom0Spec;
impl crate::RegisterSpec for Tom0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tom0::R`](R) reader structure"]
impl crate::Readable for Tom0Spec {}
#[doc = "`write(|w| ..)` method takes [`tom0::W`](W) writer structure"]
impl crate::Writable for Tom0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TOM0 to value 0"]
impl crate::Resettable for Tom0Spec {
    const RESET_VALUE: u16 = 0;
}
