#[doc = "Register `WDTRCR` reader"]
pub type R = crate::R<WdtrcrSpec>;
#[doc = "Register `WDTRCR` writer"]
pub type W = crate::W<WdtrcrSpec>;
#[doc = "WDT Behavior Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstirqs {
    #[doc = "0: Interrupt"]
    _0 = 0,
    #[doc = "1: Reset"]
    _1 = 1,
}
impl From<Rstirqs> for bool {
    #[inline(always)]
    fn from(variant: Rstirqs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTIRQS` reader - WDT Behavior Selection"]
pub type RstirqsR = crate::BitReader<Rstirqs>;
impl RstirqsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstirqs {
        match self.bits {
            false => Rstirqs::_0,
            true => Rstirqs::_1,
        }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rstirqs::_0
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rstirqs::_1
    }
}
#[doc = "Field `RSTIRQS` writer - WDT Behavior Selection"]
pub type RstirqsW<'a, REG> = crate::BitWriter<'a, REG, Rstirqs>;
impl<'a, REG> RstirqsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rstirqs::_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rstirqs::_1)
    }
}
impl R {
    #[doc = "Bit 7 - WDT Behavior Selection"]
    #[inline(always)]
    pub fn rstirqs(&self) -> RstirqsR {
        RstirqsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - WDT Behavior Selection"]
    #[inline(always)]
    pub fn rstirqs(&mut self) -> RstirqsW<WdtrcrSpec> {
        RstirqsW::new(self, 7)
    }
}
#[doc = "WDT Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtrcrSpec;
impl crate::RegisterSpec for WdtrcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdtrcr::R`](R) reader structure"]
impl crate::Readable for WdtrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtrcr::W`](W) writer structure"]
impl crate::Writable for WdtrcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WDTRCR to value 0x80"]
impl crate::Resettable for WdtrcrSpec {
    const RESET_VALUE: u8 = 0x80;
}
