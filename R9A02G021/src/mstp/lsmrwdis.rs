#[doc = "Register `LSMRWDIS` reader"]
pub type R = crate::R<LsmrwdisSpec>;
#[doc = "Register `LSMRWDIS` writer"]
pub type W = crate::W<LsmrwdisSpec>;
#[doc = "RTC Register R/W Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcrwdis {
    #[doc = "0: RTC register R/W clock always on"]
    _0 = 0,
    #[doc = "1: RTC register R/W clock stops"]
    _1 = 1,
}
impl From<Rtcrwdis> for bool {
    #[inline(always)]
    fn from(variant: Rtcrwdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCRWDIS` reader - RTC Register R/W Enable Control"]
pub type RtcrwdisR = crate::BitReader<Rtcrwdis>;
impl RtcrwdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcrwdis {
        match self.bits {
            false => Rtcrwdis::_0,
            true => Rtcrwdis::_1,
        }
    }
    #[doc = "RTC register R/W clock always on"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcrwdis::_0
    }
    #[doc = "RTC register R/W clock stops"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcrwdis::_1
    }
}
#[doc = "Field `RTCRWDIS` writer - RTC Register R/W Enable Control"]
pub type RtcrwdisW<'a, REG> = crate::BitWriter<'a, REG, Rtcrwdis>;
impl<'a, REG> RtcrwdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC register R/W clock always on"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcrwdis::_0)
    }
    #[doc = "RTC register R/W clock stops"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcrwdis::_1)
    }
}
#[doc = "WDT Operate Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtdis {
    #[doc = "0: WDT operates as normal"]
    _0 = 0,
    #[doc = "1: Stop the WDT clock and register R/W clock"]
    _1 = 1,
}
impl From<Wdtdis> for bool {
    #[inline(always)]
    fn from(variant: Wdtdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTDIS` reader - WDT Operate Clock Control"]
pub type WdtdisR = crate::BitReader<Wdtdis>;
impl WdtdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtdis {
        match self.bits {
            false => Wdtdis::_0,
            true => Wdtdis::_1,
        }
    }
    #[doc = "WDT operates as normal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdtdis::_0
    }
    #[doc = "Stop the WDT clock and register R/W clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdtdis::_1
    }
}
#[doc = "Field `WDTDIS` writer - WDT Operate Clock Control"]
pub type WdtdisW<'a, REG> = crate::BitWriter<'a, REG, Wdtdis>;
impl<'a, REG> WdtdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDT operates as normal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtdis::_0)
    }
    #[doc = "Stop the WDT clock and register R/W clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtdis::_1)
    }
}
#[doc = "IWDT Register Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtdis {
    #[doc = "0: IWDT operates as normal"]
    _0 = 0,
    #[doc = "1: Stop the IWDT register R/W clock"]
    _1 = 1,
}
impl From<Iwdtdis> for bool {
    #[inline(always)]
    fn from(variant: Iwdtdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDTDIS` reader - IWDT Register Clock Control"]
pub type IwdtdisR = crate::BitReader<Iwdtdis>;
impl IwdtdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtdis {
        match self.bits {
            false => Iwdtdis::_0,
            true => Iwdtdis::_1,
        }
    }
    #[doc = "IWDT operates as normal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtdis::_0
    }
    #[doc = "Stop the IWDT register R/W clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtdis::_1
    }
}
#[doc = "Field `IWDTDIS` writer - IWDT Register Clock Control"]
pub type IwdtdisW<'a, REG> = crate::BitWriter<'a, REG, Iwdtdis>;
impl<'a, REG> IwdtdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IWDT operates as normal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtdis::_0)
    }
    #[doc = "Stop the IWDT register R/W clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtdis::_1)
    }
}
#[doc = "Write Enable for bits \\[2:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wren {
    #[doc = "0: Write protect for bits \\[2:0\\]"]
    _0 = 0,
    #[doc = "1: Write enable for bits \\[2:0\\]"]
    _1 = 1,
}
impl From<Wren> for bool {
    #[inline(always)]
    fn from(variant: Wren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WREN` reader - Write Enable for bits \\[2:0\\]"]
pub type WrenR = crate::BitReader<Wren>;
impl WrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wren {
        match self.bits {
            false => Wren::_0,
            true => Wren::_1,
        }
    }
    #[doc = "Write protect for bits \\[2:0\\]"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wren::_0
    }
    #[doc = "Write enable for bits \\[2:0\\]"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wren::_1
    }
}
#[doc = "Field `WREN` writer - Write Enable for bits \\[2:0\\]"]
pub type WrenW<'a, REG> = crate::BitWriter<'a, REG, Wren>;
impl<'a, REG> WrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protect for bits \\[2:0\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wren::_0)
    }
    #[doc = "Write enable for bits \\[2:0\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wren::_1)
    }
}
#[doc = "Field `PRKEY` writer - LSMRWDIS Key Code"]
pub type PrkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - RTC Register R/W Enable Control"]
    #[inline(always)]
    pub fn rtcrwdis(&self) -> RtcrwdisR {
        RtcrwdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT Operate Clock Control"]
    #[inline(always)]
    pub fn wdtdis(&self) -> WdtdisR {
        WdtdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IWDT Register Clock Control"]
    #[inline(always)]
    pub fn iwdtdis(&self) -> IwdtdisR {
        IwdtdisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Enable for bits \\[2:0\\]"]
    #[inline(always)]
    pub fn wren(&self) -> WrenR {
        WrenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Register R/W Enable Control"]
    #[inline(always)]
    pub fn rtcrwdis(&mut self) -> RtcrwdisW<LsmrwdisSpec> {
        RtcrwdisW::new(self, 0)
    }
    #[doc = "Bit 1 - WDT Operate Clock Control"]
    #[inline(always)]
    pub fn wdtdis(&mut self) -> WdtdisW<LsmrwdisSpec> {
        WdtdisW::new(self, 1)
    }
    #[doc = "Bit 2 - IWDT Register Clock Control"]
    #[inline(always)]
    pub fn iwdtdis(&mut self) -> IwdtdisW<LsmrwdisSpec> {
        IwdtdisW::new(self, 2)
    }
    #[doc = "Bit 7 - Write Enable for bits \\[2:0\\]"]
    #[inline(always)]
    pub fn wren(&mut self) -> WrenW<LsmrwdisSpec> {
        WrenW::new(self, 7)
    }
    #[doc = "Bits 8:15 - LSMRWDIS Key Code"]
    #[inline(always)]
    pub fn prkey(&mut self) -> PrkeyW<LsmrwdisSpec> {
        PrkeyW::new(self, 8)
    }
}
#[doc = "Low Speed Module R/W Disable Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsmrwdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsmrwdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsmrwdisSpec;
impl crate::RegisterSpec for LsmrwdisSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lsmrwdis::R`](R) reader structure"]
impl crate::Readable for LsmrwdisSpec {}
#[doc = "`write(|w| ..)` method takes [`lsmrwdis::W`](W) writer structure"]
impl crate::Writable for LsmrwdisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets LSMRWDIS to value 0"]
impl crate::Resettable for LsmrwdisSpec {
    const RESET_VALUE: u16 = 0;
}
