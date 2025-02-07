#[doc = "Register `PFBER` reader"]
pub type R = crate::R<PfberSpec>;
#[doc = "Register `PFBER` writer"]
pub type W = crate::W<PfberSpec>;
#[doc = "Prefetch Buffer Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfbe {
    #[doc = "0: Prefetch buffer is disabled"]
    _0 = 0,
    #[doc = "1: Prefetch buffer is enabled"]
    _1 = 1,
}
impl From<Pfbe> for bool {
    #[inline(always)]
    fn from(variant: Pfbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFBE` reader - Prefetch Buffer Enable bit"]
pub type PfbeR = crate::BitReader<Pfbe>;
impl PfbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfbe {
        match self.bits {
            false => Pfbe::_0,
            true => Pfbe::_1,
        }
    }
    #[doc = "Prefetch buffer is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pfbe::_0
    }
    #[doc = "Prefetch buffer is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pfbe::_1
    }
}
#[doc = "Field `PFBE` writer - Prefetch Buffer Enable bit"]
pub type PfbeW<'a, REG> = crate::BitWriter<'a, REG, Pfbe>;
impl<'a, REG> PfbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prefetch buffer is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pfbe::_0)
    }
    #[doc = "Prefetch buffer is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pfbe::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Prefetch Buffer Enable bit"]
    #[inline(always)]
    pub fn pfbe(&self) -> PfbeR {
        PfbeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Prefetch Buffer Enable bit"]
    #[inline(always)]
    pub fn pfbe(&mut self) -> PfbeW<PfberSpec> {
        PfbeW::new(self, 0)
    }
}
#[doc = "Prefetch Buffer Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfberSpec;
impl crate::RegisterSpec for PfberSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pfber::R`](R) reader structure"]
impl crate::Readable for PfberSpec {}
#[doc = "`write(|w| ..)` method takes [`pfber::W`](W) writer structure"]
impl crate::Writable for PfberSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PFBER to value 0"]
impl crate::Resettable for PfberSpec {
    const RESET_VALUE: u8 = 0;
}
