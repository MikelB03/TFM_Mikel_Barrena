#[doc = "Register `TIS0` reader"]
pub type R = crate::R<Tis0Spec>;
#[doc = "Register `TIS0` writer"]
pub type W = crate::W<Tis0Spec>;
#[doc = "Selection of timer input used with channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tis {
    #[doc = "0: Input signal of timer input pin (TI05)"]
    _000 = 0,
    #[doc = "3: Middle-speed on-chip oscillator (MOCO)"]
    _011 = 3,
    #[doc = "4: Low-speed on-chip oscillator (LOCO)"]
    _100 = 4,
    #[doc = "5: Sub-clock oscillator (SOSC)"]
    _101 = 5,
    #[doc = "1: Setting prohibited"]
    Others = 1,
}
impl From<Tis> for u8 {
    #[inline(always)]
    fn from(variant: Tis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tis {
    type Ux = u8;
}
impl crate::IsEnum for Tis {}
#[doc = "Field `TIS` reader - Selection of timer input used with channel 5"]
pub type TisR = crate::FieldReader<Tis>;
impl TisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tis {
        match self.bits {
            0 => Tis::_000,
            3 => Tis::_011,
            4 => Tis::_100,
            5 => Tis::_101,
            _ => Tis::Others,
        }
    }
    #[doc = "Input signal of timer input pin (TI05)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Tis::_000
    }
    #[doc = "Middle-speed on-chip oscillator (MOCO)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Tis::_011
    }
    #[doc = "Low-speed on-chip oscillator (LOCO)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Tis::_100
    }
    #[doc = "Sub-clock oscillator (SOSC)"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Tis::_101
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Tis::Others)
    }
}
#[doc = "Field `TIS` writer - Selection of timer input used with channel 5"]
pub type TisW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tis, crate::Safe>;
impl<'a, REG> TisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input signal of timer input pin (TI05)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Tis::_000)
    }
    #[doc = "Middle-speed on-chip oscillator (MOCO)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Tis::_011)
    }
    #[doc = "Low-speed on-chip oscillator (LOCO)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Tis::_100)
    }
    #[doc = "Sub-clock oscillator (SOSC)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Tis::_101)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Tis::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selection of timer input used with channel 5"]
    #[inline(always)]
    pub fn tis(&self) -> TisR {
        TisR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection of timer input used with channel 5"]
    #[inline(always)]
    pub fn tis(&mut self) -> TisW<Tis0Spec> {
        TisW::new(self, 0)
    }
}
#[doc = "Timer Input Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tis0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tis0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tis0Spec;
impl crate::RegisterSpec for Tis0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tis0::R`](R) reader structure"]
impl crate::Readable for Tis0Spec {}
#[doc = "`write(|w| ..)` method takes [`tis0::W`](W) writer structure"]
impl crate::Writable for Tis0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TIS0 to value 0"]
impl crate::Resettable for Tis0Spec {
    const RESET_VALUE: u8 = 0;
}
