#[doc = "Register `DACR` reader"]
pub type R = crate::R<DacrSpec>;
#[doc = "Register `DACR` writer"]
pub type W = crate::W<DacrSpec>;
#[doc = "D/A Output Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daoe0 {
    #[doc = "0: Analog output of channel 0 (DA0) is disabled."]
    _0 = 0,
    #[doc = "1: D/A conversion of channel 0 is enabled. Analog output of channel 0 (DA0) is enabled."]
    _1 = 1,
}
impl From<Daoe0> for bool {
    #[inline(always)]
    fn from(variant: Daoe0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAOE0` reader - D/A Output Enable 0"]
pub type Daoe0R = crate::BitReader<Daoe0>;
impl Daoe0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Daoe0 {
        match self.bits {
            false => Daoe0::_0,
            true => Daoe0::_1,
        }
    }
    #[doc = "Analog output of channel 0 (DA0) is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daoe0::_0
    }
    #[doc = "D/A conversion of channel 0 is enabled. Analog output of channel 0 (DA0) is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daoe0::_1
    }
}
#[doc = "Field `DAOE0` writer - D/A Output Enable 0"]
pub type Daoe0W<'a, REG> = crate::BitWriter<'a, REG, Daoe0>;
impl<'a, REG> Daoe0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog output of channel 0 (DA0) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daoe0::_0)
    }
    #[doc = "D/A conversion of channel 0 is enabled. Analog output of channel 0 (DA0) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daoe0::_1)
    }
}
#[doc = "D/A Output Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daoe1 {
    #[doc = "0: Analog output of channel 1 (DA1) is disabled."]
    _0 = 0,
    #[doc = "1: D/A conversion of channel 1 is enabled. Analog output of channel 1 (DA1) is enabled."]
    _1 = 1,
}
impl From<Daoe1> for bool {
    #[inline(always)]
    fn from(variant: Daoe1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAOE1` reader - D/A Output Enable 1"]
pub type Daoe1R = crate::BitReader<Daoe1>;
impl Daoe1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Daoe1 {
        match self.bits {
            false => Daoe1::_0,
            true => Daoe1::_1,
        }
    }
    #[doc = "Analog output of channel 1 (DA1) is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daoe1::_0
    }
    #[doc = "D/A conversion of channel 1 is enabled. Analog output of channel 1 (DA1) is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daoe1::_1
    }
}
#[doc = "Field `DAOE1` writer - D/A Output Enable 1"]
pub type Daoe1W<'a, REG> = crate::BitWriter<'a, REG, Daoe1>;
impl<'a, REG> Daoe1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog output of channel 1 (DA1) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daoe1::_0)
    }
    #[doc = "D/A conversion of channel 1 is enabled. Analog output of channel 1 (DA1) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daoe1::_1)
    }
}
impl R {
    #[doc = "Bit 6 - D/A Output Enable 0"]
    #[inline(always)]
    pub fn daoe0(&self) -> Daoe0R {
        Daoe0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - D/A Output Enable 1"]
    #[inline(always)]
    pub fn daoe1(&self) -> Daoe1R {
        Daoe1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - D/A Output Enable 0"]
    #[inline(always)]
    pub fn daoe0(&mut self) -> Daoe0W<DacrSpec> {
        Daoe0W::new(self, 6)
    }
    #[doc = "Bit 7 - D/A Output Enable 1"]
    #[inline(always)]
    pub fn daoe1(&mut self) -> Daoe1W<DacrSpec> {
        Daoe1W::new(self, 7)
    }
}
#[doc = "D/A Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacrSpec;
impl crate::RegisterSpec for DacrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dacr::R`](R) reader structure"]
impl crate::Readable for DacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dacr::W`](W) writer structure"]
impl crate::Writable for DacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DACR to value 0x1f"]
impl crate::Resettable for DacrSpec {
    const RESET_VALUE: u8 = 0x1f;
}
