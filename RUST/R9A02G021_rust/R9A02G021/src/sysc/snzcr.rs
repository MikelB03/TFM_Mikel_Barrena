#[doc = "Register `SNZCR` reader"]
pub type R = crate::R<SnzcrSpec>;
#[doc = "Register `SNZCR` writer"]
pub type W = crate::W<SnzcrSpec>;
#[doc = "DTC Enable in Snooze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzdtcen {
    #[doc = "0: Disable DTC operation"]
    _0 = 0,
    #[doc = "1: Enable DTC operation"]
    _1 = 1,
}
impl From<Snzdtcen> for bool {
    #[inline(always)]
    fn from(variant: Snzdtcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZDTCEN` reader - DTC Enable in Snooze mode"]
pub type SnzdtcenR = crate::BitReader<Snzdtcen>;
impl SnzdtcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzdtcen {
        match self.bits {
            false => Snzdtcen::_0,
            true => Snzdtcen::_1,
        }
    }
    #[doc = "Disable DTC operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzdtcen::_0
    }
    #[doc = "Enable DTC operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzdtcen::_1
    }
}
#[doc = "Field `SNZDTCEN` writer - DTC Enable in Snooze mode"]
pub type SnzdtcenW<'a, REG> = crate::BitWriter<'a, REG, Snzdtcen>;
impl<'a, REG> SnzdtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DTC operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzdtcen::_0)
    }
    #[doc = "Enable DTC operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzdtcen::_1)
    }
}
#[doc = "RXD0 or SCK00 Snooze Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxd0reqen {
    #[doc = "0: Ignore RXD0 or SCK00 edge in Software Standby mode"]
    _0 = 0,
    #[doc = "1: Detect RXD0 or SCK00 edge in Software Standby mode"]
    _1 = 1,
}
impl From<Rxd0reqen> for bool {
    #[inline(always)]
    fn from(variant: Rxd0reqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXD0REQEN` reader - RXD0 or SCK00 Snooze Request Enable"]
pub type Rxd0reqenR = crate::BitReader<Rxd0reqen>;
impl Rxd0reqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxd0reqen {
        match self.bits {
            false => Rxd0reqen::_0,
            true => Rxd0reqen::_1,
        }
    }
    #[doc = "Ignore RXD0 or SCK00 edge in Software Standby mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxd0reqen::_0
    }
    #[doc = "Detect RXD0 or SCK00 edge in Software Standby mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxd0reqen::_1
    }
}
#[doc = "Field `RXD0REQEN` writer - RXD0 or SCK00 Snooze Request Enable"]
pub type Rxd0reqenW<'a, REG> = crate::BitWriter<'a, REG, Rxd0reqen>;
impl<'a, REG> Rxd0reqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore RXD0 or SCK00 edge in Software Standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd0reqen::_0)
    }
    #[doc = "Detect RXD0 or SCK00 edge in Software Standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd0reqen::_1)
    }
}
#[doc = "RXD2 or SCK20 Snooze Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxd2reqen {
    #[doc = "0: Ignore RXD2 or SCK20 edge in Software Standby mode"]
    _0 = 0,
    #[doc = "1: Detect RXD2 or SCK20 edge in Software Standby mode"]
    _1 = 1,
}
impl From<Rxd2reqen> for bool {
    #[inline(always)]
    fn from(variant: Rxd2reqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXD2REQEN` reader - RXD2 or SCK20 Snooze Request Enable"]
pub type Rxd2reqenR = crate::BitReader<Rxd2reqen>;
impl Rxd2reqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxd2reqen {
        match self.bits {
            false => Rxd2reqen::_0,
            true => Rxd2reqen::_1,
        }
    }
    #[doc = "Ignore RXD2 or SCK20 edge in Software Standby mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxd2reqen::_0
    }
    #[doc = "Detect RXD2 or SCK20 edge in Software Standby mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxd2reqen::_1
    }
}
#[doc = "Field `RXD2REQEN` writer - RXD2 or SCK20 Snooze Request Enable"]
pub type Rxd2reqenW<'a, REG> = crate::BitWriter<'a, REG, Rxd2reqen>;
impl<'a, REG> Rxd2reqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore RXD2 or SCK20 edge in Software Standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd2reqen::_0)
    }
    #[doc = "Detect RXD2 or SCK20 edge in Software Standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd2reqen::_1)
    }
}
#[doc = "RIN0 Snooze Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Remc0reqen {
    #[doc = "0: Ignore RIN0 edge in Software Standby mode"]
    _0 = 0,
    #[doc = "1: Detect RIN0 edge in Software Standby mode"]
    _1 = 1,
}
impl From<Remc0reqen> for bool {
    #[inline(always)]
    fn from(variant: Remc0reqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REMC0REQEN` reader - RIN0 Snooze Request Enable"]
pub type Remc0reqenR = crate::BitReader<Remc0reqen>;
impl Remc0reqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Remc0reqen {
        match self.bits {
            false => Remc0reqen::_0,
            true => Remc0reqen::_1,
        }
    }
    #[doc = "Ignore RIN0 edge in Software Standby mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Remc0reqen::_0
    }
    #[doc = "Detect RIN0 edge in Software Standby mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Remc0reqen::_1
    }
}
#[doc = "Field `REMC0REQEN` writer - RIN0 Snooze Request Enable"]
pub type Remc0reqenW<'a, REG> = crate::BitWriter<'a, REG, Remc0reqen>;
impl<'a, REG> Remc0reqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore RIN0 edge in Software Standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Remc0reqen::_0)
    }
    #[doc = "Detect RIN0 edge in Software Standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Remc0reqen::_1)
    }
}
#[doc = "Snooze mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snze {
    #[doc = "0: Disable Snooze mode"]
    _0 = 0,
    #[doc = "1: Enable Snooze mode"]
    _1 = 1,
}
impl From<Snze> for bool {
    #[inline(always)]
    fn from(variant: Snze) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZE` reader - Snooze mode Enable"]
pub type SnzeR = crate::BitReader<Snze>;
impl SnzeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snze {
        match self.bits {
            false => Snze::_0,
            true => Snze::_1,
        }
    }
    #[doc = "Disable Snooze mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snze::_0
    }
    #[doc = "Enable Snooze mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snze::_1
    }
}
#[doc = "Field `SNZE` writer - Snooze mode Enable"]
pub type SnzeW<'a, REG> = crate::BitWriter<'a, REG, Snze>;
impl<'a, REG> SnzeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Snooze mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snze::_0)
    }
    #[doc = "Enable Snooze mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snze::_1)
    }
}
impl R {
    #[doc = "Bit 1 - DTC Enable in Snooze mode"]
    #[inline(always)]
    pub fn snzdtcen(&self) -> SnzdtcenR {
        SnzdtcenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXD0 or SCK00 Snooze Request Enable"]
    #[inline(always)]
    pub fn rxd0reqen(&self) -> Rxd0reqenR {
        Rxd0reqenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXD2 or SCK20 Snooze Request Enable"]
    #[inline(always)]
    pub fn rxd2reqen(&self) -> Rxd2reqenR {
        Rxd2reqenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RIN0 Snooze Request Enable"]
    #[inline(always)]
    pub fn remc0reqen(&self) -> Remc0reqenR {
        Remc0reqenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Snooze mode Enable"]
    #[inline(always)]
    pub fn snze(&self) -> SnzeR {
        SnzeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DTC Enable in Snooze mode"]
    #[inline(always)]
    pub fn snzdtcen(&mut self) -> SnzdtcenW<SnzcrSpec> {
        SnzdtcenW::new(self, 1)
    }
    #[doc = "Bit 2 - RXD0 or SCK00 Snooze Request Enable"]
    #[inline(always)]
    pub fn rxd0reqen(&mut self) -> Rxd0reqenW<SnzcrSpec> {
        Rxd0reqenW::new(self, 2)
    }
    #[doc = "Bit 3 - RXD2 or SCK20 Snooze Request Enable"]
    #[inline(always)]
    pub fn rxd2reqen(&mut self) -> Rxd2reqenW<SnzcrSpec> {
        Rxd2reqenW::new(self, 3)
    }
    #[doc = "Bit 4 - RIN0 Snooze Request Enable"]
    #[inline(always)]
    pub fn remc0reqen(&mut self) -> Remc0reqenW<SnzcrSpec> {
        Remc0reqenW::new(self, 4)
    }
    #[doc = "Bit 7 - Snooze mode Enable"]
    #[inline(always)]
    pub fn snze(&mut self) -> SnzeW<SnzcrSpec> {
        SnzeW::new(self, 7)
    }
}
#[doc = "Snooze Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snzcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnzcrSpec;
impl crate::RegisterSpec for SnzcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`snzcr::R`](R) reader structure"]
impl crate::Readable for SnzcrSpec {}
#[doc = "`write(|w| ..)` method takes [`snzcr::W`](W) writer structure"]
impl crate::Writable for SnzcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SNZCR to value 0"]
impl crate::Resettable for SnzcrSpec {
    const RESET_VALUE: u8 = 0;
}
