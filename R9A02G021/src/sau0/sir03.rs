#[doc = "Register `SIR03` reader"]
pub type R = crate::R<Sir03Spec>;
#[doc = "Register `SIR03` writer"]
pub type W = crate::W<Sir03Spec>;
#[doc = "Clear trigger of overrun error flag of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovct {
    #[doc = "0: Not cleared"]
    _0 = 0,
    #[doc = "1: Clears the OVF flag of the SSRmn register to 0"]
    _1 = 1,
}
impl From<Ovct> for bool {
    #[inline(always)]
    fn from(variant: Ovct) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVCT` reader - Clear trigger of overrun error flag of channel n"]
pub type OvctR = crate::BitReader<Ovct>;
impl OvctR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovct {
        match self.bits {
            false => Ovct::_0,
            true => Ovct::_1,
        }
    }
    #[doc = "Not cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovct::_0
    }
    #[doc = "Clears the OVF flag of the SSRmn register to 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovct::_1
    }
}
#[doc = "Field `OVCT` writer - Clear trigger of overrun error flag of channel n"]
pub type OvctW<'a, REG> = crate::BitWriter<'a, REG, Ovct>;
impl<'a, REG> OvctW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovct::_0)
    }
    #[doc = "Clears the OVF flag of the SSRmn register to 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovct::_1)
    }
}
#[doc = "Clear trigger of parity error flag of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pect {
    #[doc = "0: Not cleared"]
    _0 = 0,
    #[doc = "1: Clears the PEF flag of the SSRmn register to 0"]
    _1 = 1,
}
impl From<Pect> for bool {
    #[inline(always)]
    fn from(variant: Pect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECT` reader - Clear trigger of parity error flag of channel n"]
pub type PectR = crate::BitReader<Pect>;
impl PectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pect {
        match self.bits {
            false => Pect::_0,
            true => Pect::_1,
        }
    }
    #[doc = "Not cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pect::_0
    }
    #[doc = "Clears the PEF flag of the SSRmn register to 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pect::_1
    }
}
#[doc = "Field `PECT` writer - Clear trigger of parity error flag of channel n"]
pub type PectW<'a, REG> = crate::BitWriter<'a, REG, Pect>;
impl<'a, REG> PectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pect::_0)
    }
    #[doc = "Clears the PEF flag of the SSRmn register to 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pect::_1)
    }
}
#[doc = "Clear trigger of framing error flag of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fect {
    #[doc = "0: Not cleared"]
    _0 = 0,
    #[doc = "1: Clears the FEF flag of the SSRmn register to 0"]
    _1 = 1,
}
impl From<Fect> for bool {
    #[inline(always)]
    fn from(variant: Fect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FECT` reader - Clear trigger of framing error flag of channel n"]
pub type FectR = crate::BitReader<Fect>;
impl FectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fect {
        match self.bits {
            false => Fect::_0,
            true => Fect::_1,
        }
    }
    #[doc = "Not cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fect::_0
    }
    #[doc = "Clears the FEF flag of the SSRmn register to 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fect::_1
    }
}
#[doc = "Field `FECT` writer - Clear trigger of framing error flag of channel n"]
pub type FectW<'a, REG> = crate::BitWriter<'a, REG, Fect>;
impl<'a, REG> FectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fect::_0)
    }
    #[doc = "Clears the FEF flag of the SSRmn register to 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fect::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clear trigger of overrun error flag of channel n"]
    #[inline(always)]
    pub fn ovct(&self) -> OvctR {
        OvctR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear trigger of parity error flag of channel n"]
    #[inline(always)]
    pub fn pect(&self) -> PectR {
        PectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear trigger of framing error flag of channel n"]
    #[inline(always)]
    pub fn fect(&self) -> FectR {
        FectR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear trigger of overrun error flag of channel n"]
    #[inline(always)]
    pub fn ovct(&mut self) -> OvctW<Sir03Spec> {
        OvctW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear trigger of parity error flag of channel n"]
    #[inline(always)]
    pub fn pect(&mut self) -> PectW<Sir03Spec> {
        PectW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear trigger of framing error flag of channel n"]
    #[inline(always)]
    pub fn fect(&mut self) -> FectW<Sir03Spec> {
        FectW::new(self, 2)
    }
}
#[doc = "Serial Flag Clear Trigger Register 03\n\nYou can [`read`](crate::Reg::read) this register and get [`sir03::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sir03::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sir03Spec;
impl crate::RegisterSpec for Sir03Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sir03::R`](R) reader structure"]
impl crate::Readable for Sir03Spec {}
#[doc = "`write(|w| ..)` method takes [`sir03::W`](W) writer structure"]
impl crate::Writable for Sir03Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SIR03 to value 0"]
impl crate::Resettable for Sir03Spec {
    const RESET_VALUE: u16 = 0;
}
