#[doc = "Register `MACTCR` reader"]
pub type R = crate::R<MactcrSpec>;
#[doc = "Register `MACTCR` writer"]
pub type W = crate::W<MactcrSpec>;
#[doc = "Machine Timer clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: No clock is supplied to Machine Timer"]
    _0 = 0,
    #[doc = "1: Clock is supplied to Machine Timer"]
    _1 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Machine Timer clock enable"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::_0,
            true => Enable::_1,
        }
    }
    #[doc = "No clock is supplied to Machine Timer"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enable::_0
    }
    #[doc = "Clock is supplied to Machine Timer"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enable::_1
    }
}
#[doc = "Field `ENABLE` writer - Machine Timer clock enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No clock is supplied to Machine Timer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_0)
    }
    #[doc = "Clock is supplied to Machine Timer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_1)
    }
}
#[doc = "Machine Timer clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clocksource {
    #[doc = "0: Machine Timer Clock"]
    _0 = 0,
    #[doc = "1: CPU Clock"]
    _1 = 1,
}
impl From<Clocksource> for bool {
    #[inline(always)]
    fn from(variant: Clocksource) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLOCKSOURCE` reader - Machine Timer clock source select"]
pub type ClocksourceR = crate::BitReader<Clocksource>;
impl ClocksourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clocksource {
        match self.bits {
            false => Clocksource::_0,
            true => Clocksource::_1,
        }
    }
    #[doc = "Machine Timer Clock"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Clocksource::_0
    }
    #[doc = "CPU Clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Clocksource::_1
    }
}
#[doc = "Field `CLOCKSOURCE` writer - Machine Timer clock source select"]
pub type ClocksourceW<'a, REG> = crate::BitWriter<'a, REG, Clocksource>;
impl<'a, REG> ClocksourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Machine Timer Clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Clocksource::_0)
    }
    #[doc = "CPU Clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Clocksource::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Machine Timer clock enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Machine Timer clock source select"]
    #[inline(always)]
    pub fn clocksource(&self) -> ClocksourceR {
        ClocksourceR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Machine Timer clock enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<MactcrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Machine Timer clock source select"]
    #[inline(always)]
    pub fn clocksource(&mut self) -> ClocksourceW<MactcrSpec> {
        ClocksourceW::new(self, 1)
    }
}
#[doc = "Machine Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MactcrSpec;
impl crate::RegisterSpec for MactcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactcr::R`](R) reader structure"]
impl crate::Readable for MactcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mactcr::W`](W) writer structure"]
impl crate::Writable for MactcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACTCR to value 0"]
impl crate::Resettable for MactcrSpec {
    const RESET_VALUE: u32 = 0;
}
