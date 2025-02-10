#[doc = "Register `RSTSR1` reader"]
pub type R = crate::R<Rstsr1Spec>;
#[doc = "Register `RSTSR1` writer"]
pub type W = crate::W<Rstsr1Spec>;
#[doc = "Independent Watchdog Timer Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtrf {
    #[doc = "0: Independent watchdog timer reset not detected"]
    _0 = 0,
    #[doc = "1: Independent watchdog timer reset detected"]
    _1 = 1,
}
impl From<Iwdtrf> for bool {
    #[inline(always)]
    fn from(variant: Iwdtrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDTRF` reader - Independent Watchdog Timer Reset Detect Flag"]
pub type IwdtrfR = crate::BitReader<Iwdtrf>;
impl IwdtrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtrf {
        match self.bits {
            false => Iwdtrf::_0,
            true => Iwdtrf::_1,
        }
    }
    #[doc = "Independent watchdog timer reset not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtrf::_0
    }
    #[doc = "Independent watchdog timer reset detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtrf::_1
    }
}
#[doc = "Field `IWDTRF` writer - Independent Watchdog Timer Reset Detect Flag"]
pub type IwdtrfW<'a, REG> = crate::BitWriter<'a, REG, Iwdtrf>;
impl<'a, REG> IwdtrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Independent watchdog timer reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtrf::_0)
    }
    #[doc = "Independent watchdog timer reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtrf::_1)
    }
}
#[doc = "Watchdog Timer Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtrf {
    #[doc = "0: Watchdog timer reset not detected"]
    _0 = 0,
    #[doc = "1: Watchdog timer reset detected"]
    _1 = 1,
}
impl From<Wdtrf> for bool {
    #[inline(always)]
    fn from(variant: Wdtrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRF` reader - Watchdog Timer Reset Detect Flag"]
pub type WdtrfR = crate::BitReader<Wdtrf>;
impl WdtrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtrf {
        match self.bits {
            false => Wdtrf::_0,
            true => Wdtrf::_1,
        }
    }
    #[doc = "Watchdog timer reset not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdtrf::_0
    }
    #[doc = "Watchdog timer reset detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdtrf::_1
    }
}
#[doc = "Field `WDTRF` writer - Watchdog Timer Reset Detect Flag"]
pub type WdtrfW<'a, REG> = crate::BitWriter<'a, REG, Wdtrf>;
impl<'a, REG> WdtrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Watchdog timer reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtrf::_0)
    }
    #[doc = "Watchdog timer reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtrf::_1)
    }
}
#[doc = "Software Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrf {
    #[doc = "0: Software reset not detected"]
    _0 = 0,
    #[doc = "1: Software reset detected"]
    _1 = 1,
}
impl From<Swrf> for bool {
    #[inline(always)]
    fn from(variant: Swrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRF` reader - Software Reset Detect Flag"]
pub type SwrfR = crate::BitReader<Swrf>;
impl SwrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrf {
        match self.bits {
            false => Swrf::_0,
            true => Swrf::_1,
        }
    }
    #[doc = "Software reset not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Swrf::_0
    }
    #[doc = "Software reset detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Swrf::_1
    }
}
#[doc = "Field `SWRF` writer - Software Reset Detect Flag"]
pub type SwrfW<'a, REG> = crate::BitWriter<'a, REG, Swrf>;
impl<'a, REG> SwrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Swrf::_0)
    }
    #[doc = "Software reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Swrf::_1)
    }
}
#[doc = "SRAM Parity Error Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rperf {
    #[doc = "0: SRAM parity error reset not detected"]
    _0 = 0,
    #[doc = "1: SRAM parity error reset detected"]
    _1 = 1,
}
impl From<Rperf> for bool {
    #[inline(always)]
    fn from(variant: Rperf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPERF` reader - SRAM Parity Error Reset Detect Flag"]
pub type RperfR = crate::BitReader<Rperf>;
impl RperfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rperf {
        match self.bits {
            false => Rperf::_0,
            true => Rperf::_1,
        }
    }
    #[doc = "SRAM parity error reset not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rperf::_0
    }
    #[doc = "SRAM parity error reset detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rperf::_1
    }
}
#[doc = "Field `RPERF` writer - SRAM Parity Error Reset Detect Flag"]
pub type RperfW<'a, REG> = crate::BitWriter<'a, REG, Rperf>;
impl<'a, REG> RperfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM parity error reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rperf::_0)
    }
    #[doc = "SRAM parity error reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rperf::_1)
    }
}
#[doc = "SRAM ECC Error Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reerf {
    #[doc = "0: SRAM ECC error reset not detected"]
    _0 = 0,
    #[doc = "1: SRAM ECC error reset detected"]
    _1 = 1,
}
impl From<Reerf> for bool {
    #[inline(always)]
    fn from(variant: Reerf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REERF` reader - SRAM ECC Error Reset Detect Flag"]
pub type ReerfR = crate::BitReader<Reerf>;
impl ReerfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reerf {
        match self.bits {
            false => Reerf::_0,
            true => Reerf::_1,
        }
    }
    #[doc = "SRAM ECC error reset not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reerf::_0
    }
    #[doc = "SRAM ECC error reset detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reerf::_1
    }
}
#[doc = "Field `REERF` writer - SRAM ECC Error Reset Detect Flag"]
pub type ReerfW<'a, REG> = crate::BitWriter<'a, REG, Reerf>;
impl<'a, REG> ReerfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM ECC error reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Reerf::_0)
    }
    #[doc = "SRAM ECC error reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Reerf::_1)
    }
}
#[doc = "Bus Error Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Buserf {
    #[doc = "0: Bus error reset not detected"]
    _0 = 0,
    #[doc = "1: Bus error reset detected"]
    _1 = 1,
}
impl From<Buserf> for bool {
    #[inline(always)]
    fn from(variant: Buserf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSERF` reader - Bus Error Reset Detect Flag"]
pub type BuserfR = crate::BitReader<Buserf>;
impl BuserfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Buserf {
        match self.bits {
            false => Buserf::_0,
            true => Buserf::_1,
        }
    }
    #[doc = "Bus error reset not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Buserf::_0
    }
    #[doc = "Bus error reset detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Buserf::_1
    }
}
#[doc = "Field `BUSERF` writer - Bus Error Reset Detect Flag"]
pub type BuserfW<'a, REG> = crate::BitWriter<'a, REG, Buserf>;
impl<'a, REG> BuserfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus error reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Buserf::_0)
    }
    #[doc = "Bus error reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Buserf::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Independent Watchdog Timer Reset Detect Flag"]
    #[inline(always)]
    pub fn iwdtrf(&self) -> IwdtrfR {
        IwdtrfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset Detect Flag"]
    #[inline(always)]
    pub fn wdtrf(&self) -> WdtrfR {
        WdtrfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset Detect Flag"]
    #[inline(always)]
    pub fn swrf(&self) -> SwrfR {
        SwrfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM Parity Error Reset Detect Flag"]
    #[inline(always)]
    pub fn rperf(&self) -> RperfR {
        RperfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM ECC Error Reset Detect Flag"]
    #[inline(always)]
    pub fn reerf(&self) -> ReerfR {
        ReerfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Bus Error Reset Detect Flag"]
    #[inline(always)]
    pub fn buserf(&self) -> BuserfR {
        BuserfR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Independent Watchdog Timer Reset Detect Flag"]
    #[inline(always)]
    pub fn iwdtrf(&mut self) -> IwdtrfW<Rstsr1Spec> {
        IwdtrfW::new(self, 0)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset Detect Flag"]
    #[inline(always)]
    pub fn wdtrf(&mut self) -> WdtrfW<Rstsr1Spec> {
        WdtrfW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset Detect Flag"]
    #[inline(always)]
    pub fn swrf(&mut self) -> SwrfW<Rstsr1Spec> {
        SwrfW::new(self, 2)
    }
    #[doc = "Bit 8 - SRAM Parity Error Reset Detect Flag"]
    #[inline(always)]
    pub fn rperf(&mut self) -> RperfW<Rstsr1Spec> {
        RperfW::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM ECC Error Reset Detect Flag"]
    #[inline(always)]
    pub fn reerf(&mut self) -> ReerfW<Rstsr1Spec> {
        ReerfW::new(self, 9)
    }
    #[doc = "Bit 12 - Bus Error Reset Detect Flag"]
    #[inline(always)]
    pub fn buserf(&mut self) -> BuserfW<Rstsr1Spec> {
        BuserfW::new(self, 12)
    }
}
#[doc = "Reset Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rstsr1Spec;
impl crate::RegisterSpec for Rstsr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rstsr1::R`](R) reader structure"]
impl crate::Readable for Rstsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rstsr1::W`](W) writer structure"]
impl crate::Writable for Rstsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RSTSR1 to value 0"]
impl crate::Resettable for Rstsr1Spec {
    const RESET_VALUE: u16 = 0;
}
