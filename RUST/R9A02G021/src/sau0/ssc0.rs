#[doc = "Register `SSC0` reader"]
pub type R = crate::R<Ssc0Spec>;
#[doc = "Register `SSC0` writer"]
pub type W = crate::W<Ssc0Spec>;
#[doc = "Setting of the Snooze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swc {
    #[doc = "0: Do not use the Snooze mode function"]
    _0 = 0,
    #[doc = "1: Use the Snooze mode function"]
    _1 = 1,
}
impl From<Swc> for bool {
    #[inline(always)]
    fn from(variant: Swc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWC` reader - Setting of the Snooze mode"]
pub type SwcR = crate::BitReader<Swc>;
impl SwcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swc {
        match self.bits {
            false => Swc::_0,
            true => Swc::_1,
        }
    }
    #[doc = "Do not use the Snooze mode function"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Swc::_0
    }
    #[doc = "Use the Snooze mode function"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Swc::_1
    }
}
#[doc = "Field `SWC` writer - Setting of the Snooze mode"]
pub type SwcW<'a, REG> = crate::BitWriter<'a, REG, Swc>;
impl<'a, REG> SwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not use the Snooze mode function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Swc::_0)
    }
    #[doc = "Use the Snooze mode function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Swc::_1)
    }
}
#[doc = "Selection of whether to enable or disable the generation of communication error interrupts in the Snooze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssec {
    #[doc = "0: Enable the generation of error interrupts SAUm_INTSREq"]
    _0 = 0,
    #[doc = "1: Disable the generation of error interrupts SAUm_INTSREq"]
    _1 = 1,
}
impl From<Ssec> for bool {
    #[inline(always)]
    fn from(variant: Ssec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEC` reader - Selection of whether to enable or disable the generation of communication error interrupts in the Snooze mode"]
pub type SsecR = crate::BitReader<Ssec>;
impl SsecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssec {
        match self.bits {
            false => Ssec::_0,
            true => Ssec::_1,
        }
    }
    #[doc = "Enable the generation of error interrupts SAUm_INTSREq"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssec::_0
    }
    #[doc = "Disable the generation of error interrupts SAUm_INTSREq"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssec::_1
    }
}
#[doc = "Field `SSEC` writer - Selection of whether to enable or disable the generation of communication error interrupts in the Snooze mode"]
pub type SsecW<'a, REG> = crate::BitWriter<'a, REG, Ssec>;
impl<'a, REG> SsecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the generation of error interrupts SAUm_INTSREq"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssec::_0)
    }
    #[doc = "Disable the generation of error interrupts SAUm_INTSREq"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssec::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Setting of the Snooze mode"]
    #[inline(always)]
    pub fn swc(&self) -> SwcR {
        SwcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selection of whether to enable or disable the generation of communication error interrupts in the Snooze mode"]
    #[inline(always)]
    pub fn ssec(&self) -> SsecR {
        SsecR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting of the Snooze mode"]
    #[inline(always)]
    pub fn swc(&mut self) -> SwcW<Ssc0Spec> {
        SwcW::new(self, 0)
    }
    #[doc = "Bit 1 - Selection of whether to enable or disable the generation of communication error interrupts in the Snooze mode"]
    #[inline(always)]
    pub fn ssec(&mut self) -> SsecW<Ssc0Spec> {
        SsecW::new(self, 1)
    }
}
#[doc = "Serial Standby Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ssc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ssc0Spec;
impl crate::RegisterSpec for Ssc0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssc0::R`](R) reader structure"]
impl crate::Readable for Ssc0Spec {}
#[doc = "`write(|w| ..)` method takes [`ssc0::W`](W) writer structure"]
impl crate::Writable for Ssc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SSC0 to value 0"]
impl crate::Resettable for Ssc0Spec {
    const RESET_VALUE: u16 = 0;
}
