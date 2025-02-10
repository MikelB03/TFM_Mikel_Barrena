#[doc = "Register `FPMCR` reader"]
pub type R = crate::R<FpmcrSpec>;
#[doc = "Register `FPMCR` writer"]
pub type W = crate::W<FpmcrSpec>;
#[doc = "Flash Operating Mode Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fms0 {
    #[doc = "0: FMS1 = 0: Read mode FMS1 = 1: Data flash P/E mode."]
    _0 = 0,
    #[doc = "1: FMS1 = 0: Code flash P/E mode FMS1 = 1: Setting prohibited."]
    _1 = 1,
}
impl From<Fms0> for bool {
    #[inline(always)]
    fn from(variant: Fms0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMS0` reader - Flash Operating Mode Select 0"]
pub type Fms0R = crate::BitReader<Fms0>;
impl Fms0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fms0 {
        match self.bits {
            false => Fms0::_0,
            true => Fms0::_1,
        }
    }
    #[doc = "FMS1 = 0: Read mode FMS1 = 1: Data flash P/E mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fms0::_0
    }
    #[doc = "FMS1 = 0: Code flash P/E mode FMS1 = 1: Setting prohibited."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fms0::_1
    }
}
#[doc = "Field `FMS0` writer - Flash Operating Mode Select 0"]
pub type Fms0W<'a, REG> = crate::BitWriter<'a, REG, Fms0>;
impl<'a, REG> Fms0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FMS1 = 0: Read mode FMS1 = 1: Data flash P/E mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fms0::_0)
    }
    #[doc = "FMS1 = 0: Code flash P/E mode FMS1 = 1: Setting prohibited."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fms0::_1)
    }
}
#[doc = "Code Flash P/E Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpdis {
    #[doc = "0: Programming of the code flash is enabled"]
    _0 = 0,
    #[doc = "1: Programming of the code flash is disabled"]
    _1 = 1,
}
impl From<Rpdis> for bool {
    #[inline(always)]
    fn from(variant: Rpdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPDIS` reader - Code Flash P/E Disable"]
pub type RpdisR = crate::BitReader<Rpdis>;
impl RpdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpdis {
        match self.bits {
            false => Rpdis::_0,
            true => Rpdis::_1,
        }
    }
    #[doc = "Programming of the code flash is enabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpdis::_0
    }
    #[doc = "Programming of the code flash is disabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpdis::_1
    }
}
#[doc = "Field `RPDIS` writer - Code Flash P/E Disable"]
pub type RpdisW<'a, REG> = crate::BitWriter<'a, REG, Rpdis>;
impl<'a, REG> RpdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programming of the code flash is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpdis::_0)
    }
    #[doc = "Programming of the code flash is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpdis::_1)
    }
}
#[doc = "Field `FMS1` reader - Flash Operating Mode Select 1"]
pub type Fms1R = crate::BitReader;
#[doc = "Field `FMS1` writer - Flash Operating Mode Select 1"]
pub type Fms1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Flash Operating Mode Select 0"]
    #[inline(always)]
    pub fn fms0(&self) -> Fms0R {
        Fms0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Code Flash P/E Disable"]
    #[inline(always)]
    pub fn rpdis(&self) -> RpdisR {
        RpdisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Operating Mode Select 1"]
    #[inline(always)]
    pub fn fms1(&self) -> Fms1R {
        Fms1R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Flash Operating Mode Select 0"]
    #[inline(always)]
    pub fn fms0(&mut self) -> Fms0W<FpmcrSpec> {
        Fms0W::new(self, 1)
    }
    #[doc = "Bit 3 - Code Flash P/E Disable"]
    #[inline(always)]
    pub fn rpdis(&mut self) -> RpdisW<FpmcrSpec> {
        RpdisW::new(self, 3)
    }
    #[doc = "Bit 4 - Flash Operating Mode Select 1"]
    #[inline(always)]
    pub fn fms1(&mut self) -> Fms1W<FpmcrSpec> {
        Fms1W::new(self, 4)
    }
}
#[doc = "Flash P/E Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpmcrSpec;
impl crate::RegisterSpec for FpmcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fpmcr::R`](R) reader structure"]
impl crate::Readable for FpmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fpmcr::W`](W) writer structure"]
impl crate::Writable for FpmcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FPMCR to value 0x08"]
impl crate::Resettable for FpmcrSpec {
    const RESET_VALUE: u8 = 0x08;
}
