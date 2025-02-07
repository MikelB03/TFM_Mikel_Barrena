#[doc = "Register `RTCC1` reader"]
pub type R = crate::R<Rtcc1Spec>;
#[doc = "Register `RTCC1` writer"]
pub type W = crate::W<Rtcc1Spec>;
#[doc = "Wait control of realtime clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwait {
    #[doc = "0: Counting proceeds"]
    _0 = 0,
    #[doc = "1: Stops the SEC to YEAR counters. Counter values are readable and writable."]
    _1 = 1,
}
impl From<Rwait> for bool {
    #[inline(always)]
    fn from(variant: Rwait) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWAIT` reader - Wait control of realtime clock"]
pub type RwaitR = crate::BitReader<Rwait>;
impl RwaitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwait {
        match self.bits {
            false => Rwait::_0,
            true => Rwait::_1,
        }
    }
    #[doc = "Counting proceeds"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rwait::_0
    }
    #[doc = "Stops the SEC to YEAR counters. Counter values are readable and writable."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rwait::_1
    }
}
#[doc = "Field `RWAIT` writer - Wait control of realtime clock"]
pub type RwaitW<'a, REG> = crate::BitWriter<'a, REG, Rwait>;
impl<'a, REG> RwaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counting proceeds"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rwait::_0)
    }
    #[doc = "Stops the SEC to YEAR counters. Counter values are readable and writable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rwait::_1)
    }
}
#[doc = "Wait status flag of realtime clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwst {
    #[doc = "0: Counting is in progress"]
    _0 = 0,
    #[doc = "1: Counter values are readable and writable"]
    _1 = 1,
}
impl From<Rwst> for bool {
    #[inline(always)]
    fn from(variant: Rwst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWST` reader - Wait status flag of realtime clock"]
pub type RwstR = crate::BitReader<Rwst>;
impl RwstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwst {
        match self.bits {
            false => Rwst::_0,
            true => Rwst::_1,
        }
    }
    #[doc = "Counting is in progress"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rwst::_0
    }
    #[doc = "Counter values are readable and writable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rwst::_1
    }
}
#[doc = "Fixed-cycle interrupt status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rifg {
    #[doc = "0: Fixed-cycle interrupt is not generated"]
    _0 = 0,
    #[doc = "1: Fixed-cycle interrupt is generated"]
    _1 = 1,
}
impl From<Rifg> for bool {
    #[inline(always)]
    fn from(variant: Rifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIFG` reader - Fixed-cycle interrupt status flag"]
pub type RifgR = crate::BitReader<Rifg>;
impl RifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rifg {
        match self.bits {
            false => Rifg::_0,
            true => Rifg::_1,
        }
    }
    #[doc = "Fixed-cycle interrupt is not generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rifg::_0
    }
    #[doc = "Fixed-cycle interrupt is generated"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rifg::_1
    }
}
#[doc = "Field `RIFG` writer - Fixed-cycle interrupt status flag"]
pub type RifgW<'a, REG> = crate::BitWriter<'a, REG, Rifg>;
impl<'a, REG> RifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed-cycle interrupt is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rifg::_0)
    }
    #[doc = "Fixed-cycle interrupt is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rifg::_1)
    }
}
#[doc = "Alarm detection status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wafg {
    #[doc = "0: Alarm mismatch"]
    _0 = 0,
    #[doc = "1: Detection of matching of alarm"]
    _1 = 1,
}
impl From<Wafg> for bool {
    #[inline(always)]
    fn from(variant: Wafg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAFG` reader - Alarm detection status flag"]
pub type WafgR = crate::BitReader<Wafg>;
impl WafgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wafg {
        match self.bits {
            false => Wafg::_0,
            true => Wafg::_1,
        }
    }
    #[doc = "Alarm mismatch"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wafg::_0
    }
    #[doc = "Detection of matching of alarm"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wafg::_1
    }
}
#[doc = "Field `WAFG` writer - Alarm detection status flag"]
pub type WafgW<'a, REG> = crate::BitWriter<'a, REG, Wafg>;
impl<'a, REG> WafgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm mismatch"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wafg::_0)
    }
    #[doc = "Detection of matching of alarm"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wafg::_1)
    }
}
#[doc = "Control of alarm interrupt (RTC_ALM_OR_PRD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Walie {
    #[doc = "0: Does not generate interrupt on matching of alarm"]
    _0 = 0,
    #[doc = "1: Generates interrupt on matching of alarm"]
    _1 = 1,
}
impl From<Walie> for bool {
    #[inline(always)]
    fn from(variant: Walie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WALIE` reader - Control of alarm interrupt (RTC_ALM_OR_PRD)"]
pub type WalieR = crate::BitReader<Walie>;
impl WalieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Walie {
        match self.bits {
            false => Walie::_0,
            true => Walie::_1,
        }
    }
    #[doc = "Does not generate interrupt on matching of alarm"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Walie::_0
    }
    #[doc = "Generates interrupt on matching of alarm"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Walie::_1
    }
}
#[doc = "Field `WALIE` writer - Control of alarm interrupt (RTC_ALM_OR_PRD)"]
pub type WalieW<'a, REG> = crate::BitWriter<'a, REG, Walie>;
impl<'a, REG> WalieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not generate interrupt on matching of alarm"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Walie::_0)
    }
    #[doc = "Generates interrupt on matching of alarm"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Walie::_1)
    }
}
#[doc = "Alarm operation control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wale {
    #[doc = "0: Match operation is invalid"]
    _0 = 0,
    #[doc = "1: Match operation is valid"]
    _1 = 1,
}
impl From<Wale> for bool {
    #[inline(always)]
    fn from(variant: Wale) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WALE` reader - Alarm operation control"]
pub type WaleR = crate::BitReader<Wale>;
impl WaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wale {
        match self.bits {
            false => Wale::_0,
            true => Wale::_1,
        }
    }
    #[doc = "Match operation is invalid"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wale::_0
    }
    #[doc = "Match operation is valid"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wale::_1
    }
}
#[doc = "Field `WALE` writer - Alarm operation control"]
pub type WaleW<'a, REG> = crate::BitWriter<'a, REG, Wale>;
impl<'a, REG> WaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match operation is invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wale::_0)
    }
    #[doc = "Match operation is valid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wale::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Wait control of realtime clock"]
    #[inline(always)]
    pub fn rwait(&self) -> RwaitR {
        RwaitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wait status flag of realtime clock"]
    #[inline(always)]
    pub fn rwst(&self) -> RwstR {
        RwstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Fixed-cycle interrupt status flag"]
    #[inline(always)]
    pub fn rifg(&self) -> RifgR {
        RifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Alarm detection status flag"]
    #[inline(always)]
    pub fn wafg(&self) -> WafgR {
        WafgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Control of alarm interrupt (RTC_ALM_OR_PRD)"]
    #[inline(always)]
    pub fn walie(&self) -> WalieR {
        WalieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Alarm operation control"]
    #[inline(always)]
    pub fn wale(&self) -> WaleR {
        WaleR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wait control of realtime clock"]
    #[inline(always)]
    pub fn rwait(&mut self) -> RwaitW<Rtcc1Spec> {
        RwaitW::new(self, 0)
    }
    #[doc = "Bit 3 - Fixed-cycle interrupt status flag"]
    #[inline(always)]
    pub fn rifg(&mut self) -> RifgW<Rtcc1Spec> {
        RifgW::new(self, 3)
    }
    #[doc = "Bit 4 - Alarm detection status flag"]
    #[inline(always)]
    pub fn wafg(&mut self) -> WafgW<Rtcc1Spec> {
        WafgW::new(self, 4)
    }
    #[doc = "Bit 6 - Control of alarm interrupt (RTC_ALM_OR_PRD)"]
    #[inline(always)]
    pub fn walie(&mut self) -> WalieW<Rtcc1Spec> {
        WalieW::new(self, 6)
    }
    #[doc = "Bit 7 - Alarm operation control"]
    #[inline(always)]
    pub fn wale(&mut self) -> WaleW<Rtcc1Spec> {
        WaleW::new(self, 7)
    }
}
#[doc = "Realtime Clock Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcc1Spec;
impl crate::RegisterSpec for Rtcc1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtcc1::R`](R) reader structure"]
impl crate::Readable for Rtcc1Spec {}
#[doc = "`write(|w| ..)` method takes [`rtcc1::W`](W) writer structure"]
impl crate::Writable for Rtcc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RTCC1 to value 0"]
impl crate::Resettable for Rtcc1Spec {
    const RESET_VALUE: u8 = 0;
}
