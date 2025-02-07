#[doc = "Register `DFLCTL` reader"]
pub type R = crate::R<DflctlSpec>;
#[doc = "Register `DFLCTL` writer"]
pub type W = crate::W<DflctlSpec>;
#[doc = "Data Flash Access Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dflen {
    #[doc = "0: Access to the data flash is disabled"]
    _0 = 0,
    #[doc = "1: Access to the data flash is enabled"]
    _1 = 1,
}
impl From<Dflen> for bool {
    #[inline(always)]
    fn from(variant: Dflen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFLEN` reader - Data Flash Access Enable"]
pub type DflenR = crate::BitReader<Dflen>;
impl DflenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dflen {
        match self.bits {
            false => Dflen::_0,
            true => Dflen::_1,
        }
    }
    #[doc = "Access to the data flash is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dflen::_0
    }
    #[doc = "Access to the data flash is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dflen::_1
    }
}
#[doc = "Field `DFLEN` writer - Data Flash Access Enable"]
pub type DflenW<'a, REG> = crate::BitWriter<'a, REG, Dflen>;
impl<'a, REG> DflenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to the data flash is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dflen::_0)
    }
    #[doc = "Access to the data flash is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dflen::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Data Flash Access Enable"]
    #[inline(always)]
    pub fn dflen(&self) -> DflenR {
        DflenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Flash Access Enable"]
    #[inline(always)]
    pub fn dflen(&mut self) -> DflenW<DflctlSpec> {
        DflenW::new(self, 0)
    }
}
#[doc = "Data Flash Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dflctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DflctlSpec;
impl crate::RegisterSpec for DflctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dflctl::R`](R) reader structure"]
impl crate::Readable for DflctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dflctl::W`](W) writer structure"]
impl crate::Writable for DflctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DFLCTL to value 0"]
impl crate::Resettable for DflctlSpec {
    const RESET_VALUE: u8 = 0;
}
