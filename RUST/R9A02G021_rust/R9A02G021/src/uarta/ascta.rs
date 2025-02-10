#[doc = "Register `ASCTA%s` reader"]
pub type R = crate::R<AsctaSpec>;
#[doc = "Register `ASCTA%s` writer"]
pub type W = crate::W<AsctaSpec>;
#[doc = "Overrun error flag clear trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovecta {
    #[doc = "0: Does not clear the ASISAn.OVEA flag (the flag is retained)"]
    _0 = 0,
    #[doc = "1: Clears the ASISAn.OVEA flag"]
    _1 = 1,
}
impl From<Ovecta> for bool {
    #[inline(always)]
    fn from(variant: Ovecta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVECTA` reader - Overrun error flag clear trigger"]
pub type OvectaR = crate::BitReader<Ovecta>;
impl OvectaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovecta {
        match self.bits {
            false => Ovecta::_0,
            true => Ovecta::_1,
        }
    }
    #[doc = "Does not clear the ASISAn.OVEA flag (the flag is retained)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovecta::_0
    }
    #[doc = "Clears the ASISAn.OVEA flag"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovecta::_1
    }
}
#[doc = "Field `OVECTA` writer - Overrun error flag clear trigger"]
pub type OvectaW<'a, REG> = crate::BitWriter<'a, REG, Ovecta>;
impl<'a, REG> OvectaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not clear the ASISAn.OVEA flag (the flag is retained)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovecta::_0)
    }
    #[doc = "Clears the ASISAn.OVEA flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovecta::_1)
    }
}
#[doc = "Framing error flag clear trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fecta {
    #[doc = "0: Does not clear the ASISAn.FEA flag (the flag is retained)"]
    _0 = 0,
    #[doc = "1: Clears the ASISAn.FEA flag"]
    _1 = 1,
}
impl From<Fecta> for bool {
    #[inline(always)]
    fn from(variant: Fecta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FECTA` reader - Framing error flag clear trigger"]
pub type FectaR = crate::BitReader<Fecta>;
impl FectaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fecta {
        match self.bits {
            false => Fecta::_0,
            true => Fecta::_1,
        }
    }
    #[doc = "Does not clear the ASISAn.FEA flag (the flag is retained)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fecta::_0
    }
    #[doc = "Clears the ASISAn.FEA flag"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fecta::_1
    }
}
#[doc = "Field `FECTA` writer - Framing error flag clear trigger"]
pub type FectaW<'a, REG> = crate::BitWriter<'a, REG, Fecta>;
impl<'a, REG> FectaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not clear the ASISAn.FEA flag (the flag is retained)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fecta::_0)
    }
    #[doc = "Clears the ASISAn.FEA flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fecta::_1)
    }
}
#[doc = "Parity error flag clear trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pecta {
    #[doc = "0: Does not clear the ASISAn.PEA flag (the flag is retained)"]
    _0 = 0,
    #[doc = "1: Clears the ASISAn.PEA flag"]
    _1 = 1,
}
impl From<Pecta> for bool {
    #[inline(always)]
    fn from(variant: Pecta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECTA` reader - Parity error flag clear trigger"]
pub type PectaR = crate::BitReader<Pecta>;
impl PectaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pecta {
        match self.bits {
            false => Pecta::_0,
            true => Pecta::_1,
        }
    }
    #[doc = "Does not clear the ASISAn.PEA flag (the flag is retained)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pecta::_0
    }
    #[doc = "Clears the ASISAn.PEA flag"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pecta::_1
    }
}
#[doc = "Field `PECTA` writer - Parity error flag clear trigger"]
pub type PectaW<'a, REG> = crate::BitWriter<'a, REG, Pecta>;
impl<'a, REG> PectaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not clear the ASISAn.PEA flag (the flag is retained)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pecta::_0)
    }
    #[doc = "Clears the ASISAn.PEA flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pecta::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Overrun error flag clear trigger"]
    #[inline(always)]
    pub fn ovecta(&self) -> OvectaR {
        OvectaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error flag clear trigger"]
    #[inline(always)]
    pub fn fecta(&self) -> FectaR {
        FectaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity error flag clear trigger"]
    #[inline(always)]
    pub fn pecta(&self) -> PectaR {
        PectaR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun error flag clear trigger"]
    #[inline(always)]
    pub fn ovecta(&mut self) -> OvectaW<AsctaSpec> {
        OvectaW::new(self, 0)
    }
    #[doc = "Bit 1 - Framing error flag clear trigger"]
    #[inline(always)]
    pub fn fecta(&mut self) -> FectaW<AsctaSpec> {
        FectaW::new(self, 1)
    }
    #[doc = "Bit 2 - Parity error flag clear trigger"]
    #[inline(always)]
    pub fn pecta(&mut self) -> PectaW<AsctaSpec> {
        PectaW::new(self, 2)
    }
}
#[doc = "Status Clear Trigger Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ascta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ascta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsctaSpec;
impl crate::RegisterSpec for AsctaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ascta::R`](R) reader structure"]
impl crate::Readable for AsctaSpec {}
#[doc = "`write(|w| ..)` method takes [`ascta::W`](W) writer structure"]
impl crate::Writable for AsctaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ASCTA%s to value 0"]
impl crate::Resettable for AsctaSpec {
    const RESET_VALUE: u8 = 0;
}
