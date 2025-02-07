#[doc = "Register `HOCOCR` reader"]
pub type R = crate::R<HococrSpec>;
#[doc = "Register `HOCOCR` writer"]
pub type W = crate::W<HococrSpec>;
#[doc = "HOCO Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hcstp {
    #[doc = "0: Operate the HOCO clock ,"]
    _0 = 0,
    #[doc = "1: Stop the HOCO clock"]
    _1 = 1,
}
impl From<Hcstp> for bool {
    #[inline(always)]
    fn from(variant: Hcstp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HCSTP` reader - HOCO Stop"]
pub type HcstpR = crate::BitReader<Hcstp>;
impl HcstpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hcstp {
        match self.bits {
            false => Hcstp::_0,
            true => Hcstp::_1,
        }
    }
    #[doc = "Operate the HOCO clock ,"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hcstp::_0
    }
    #[doc = "Stop the HOCO clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hcstp::_1
    }
}
#[doc = "Field `HCSTP` writer - HOCO Stop"]
pub type HcstpW<'a, REG> = crate::BitWriter<'a, REG, Hcstp>;
impl<'a, REG> HcstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operate the HOCO clock ,"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hcstp::_0)
    }
    #[doc = "Stop the HOCO clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hcstp::_1)
    }
}
impl R {
    #[doc = "Bit 0 - HOCO Stop"]
    #[inline(always)]
    pub fn hcstp(&self) -> HcstpR {
        HcstpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HOCO Stop"]
    #[inline(always)]
    pub fn hcstp(&mut self) -> HcstpW<HococrSpec> {
        HcstpW::new(self, 0)
    }
}
#[doc = "High-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hococr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hococr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HococrSpec;
impl crate::RegisterSpec for HococrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hococr::R`](R) reader structure"]
impl crate::Readable for HococrSpec {}
#[doc = "`write(|w| ..)` method takes [`hococr::W`](W) writer structure"]
impl crate::Writable for HococrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HOCOCR to value 0"]
impl crate::Resettable for HococrSpec {
    const RESET_VALUE: u8 = 0;
}
