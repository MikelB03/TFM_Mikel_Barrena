#[doc = "Register `RTCC0` reader"]
pub type R = crate::R<Rtcc0Spec>;
#[doc = "Register `RTCC0` writer"]
pub type W = crate::W<Rtcc0Spec>;
#[doc = "Fixed-cycle interrupt (RTC_ALM_OR_PRD) selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ct {
    #[doc = "0: Does not use fixed-cycle interrupt"]
    _000 = 0,
    #[doc = "1: Once per 0.5 s (synchronized with second count up)"]
    _001 = 1,
    #[doc = "2: Once per 1 s (same time as second count up)"]
    _010 = 2,
    #[doc = "3: Once per 1 m (second 00 of every minute)"]
    _011 = 3,
    #[doc = "4: Once per 1 hour (minute 00 and second 00 of every hour)"]
    _100 = 4,
    #[doc = "5: Once per 1 day (hour 00, minute 00, and second 00 of every day)"]
    _101 = 5,
    #[doc = "6: Once per 1 month (Day 1, hour 00 a.m., minute 00, and second 00 of every month)"]
    Others = 6,
}
impl From<Ct> for u8 {
    #[inline(always)]
    fn from(variant: Ct) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ct {
    type Ux = u8;
}
impl crate::IsEnum for Ct {}
#[doc = "Field `CT` reader - Fixed-cycle interrupt (RTC_ALM_OR_PRD) selection"]
pub type CtR = crate::FieldReader<Ct>;
impl CtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct {
        match self.bits {
            0 => Ct::_000,
            1 => Ct::_001,
            2 => Ct::_010,
            3 => Ct::_011,
            4 => Ct::_100,
            5 => Ct::_101,
            _ => Ct::Others,
        }
    }
    #[doc = "Does not use fixed-cycle interrupt"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ct::_000
    }
    #[doc = "Once per 0.5 s (synchronized with second count up)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ct::_001
    }
    #[doc = "Once per 1 s (same time as second count up)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ct::_010
    }
    #[doc = "Once per 1 m (second 00 of every minute)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ct::_011
    }
    #[doc = "Once per 1 hour (minute 00 and second 00 of every hour)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ct::_100
    }
    #[doc = "Once per 1 day (hour 00, minute 00, and second 00 of every day)"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ct::_101
    }
    #[doc = "Once per 1 month (Day 1, hour 00 a.m., minute 00, and second 00 of every month)"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ct::Others)
    }
}
#[doc = "Field `CT` writer - Fixed-cycle interrupt (RTC_ALM_OR_PRD) selection"]
pub type CtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ct, crate::Safe>;
impl<'a, REG> CtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Does not use fixed-cycle interrupt"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ct::_000)
    }
    #[doc = "Once per 0.5 s (synchronized with second count up)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ct::_001)
    }
    #[doc = "Once per 1 s (same time as second count up)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ct::_010)
    }
    #[doc = "Once per 1 m (second 00 of every minute)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ct::_011)
    }
    #[doc = "Once per 1 hour (minute 00 and second 00 of every hour)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ct::_100)
    }
    #[doc = "Once per 1 day (hour 00, minute 00, and second 00 of every day)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Ct::_101)
    }
    #[doc = "Once per 1 month (Day 1, hour 00 a.m., minute 00, and second 00 of every month)"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ct::Others)
    }
}
#[doc = "Selection of 12- or 24-hour system\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ampm {
    #[doc = "0: 12-hour system (a.m. and p.m. are displayed.)"]
    _0 = 0,
    #[doc = "1: 24-hour system"]
    _1 = 1,
}
impl From<Ampm> for bool {
    #[inline(always)]
    fn from(variant: Ampm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPM` reader - Selection of 12- or 24-hour system"]
pub type AmpmR = crate::BitReader<Ampm>;
impl AmpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ampm {
        match self.bits {
            false => Ampm::_0,
            true => Ampm::_1,
        }
    }
    #[doc = "12-hour system (a.m. and p.m. are displayed.)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ampm::_0
    }
    #[doc = "24-hour system"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ampm::_1
    }
}
#[doc = "Field `AMPM` writer - Selection of 12- or 24-hour system"]
pub type AmpmW<'a, REG> = crate::BitWriter<'a, REG, Ampm>;
impl<'a, REG> AmpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "12-hour system (a.m. and p.m. are displayed.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ampm::_0)
    }
    #[doc = "24-hour system"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ampm::_1)
    }
}
#[doc = "Selection of the operating clock for the realtime clock (RTCSCLK/RTCS128CLK)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtc128en {
    #[doc = "0: RTCSCLK (32.768 kHz)"]
    _0 = 0,
    #[doc = "1: RTCS128CLK (128 Hz)"]
    _1 = 1,
}
impl From<Rtc128en> for bool {
    #[inline(always)]
    fn from(variant: Rtc128en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC128EN` reader - Selection of the operating clock for the realtime clock (RTCSCLK/RTCS128CLK)"]
pub type Rtc128enR = crate::BitReader<Rtc128en>;
impl Rtc128enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtc128en {
        match self.bits {
            false => Rtc128en::_0,
            true => Rtc128en::_1,
        }
    }
    #[doc = "RTCSCLK (32.768 kHz)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtc128en::_0
    }
    #[doc = "RTCS128CLK (128 Hz)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtc128en::_1
    }
}
#[doc = "Field `RTC128EN` writer - Selection of the operating clock for the realtime clock (RTCSCLK/RTCS128CLK)"]
pub type Rtc128enW<'a, REG> = crate::BitWriter<'a, REG, Rtc128en>;
impl<'a, REG> Rtc128enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTCSCLK (32.768 kHz)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc128en::_0)
    }
    #[doc = "RTCS128CLK (128 Hz)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc128en::_1)
    }
}
#[doc = "RTC1HZ pin output control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcloe1 {
    #[doc = "0: Disables output of the RTC1HZ pin (1 Hz)"]
    _0 = 0,
    #[doc = "1: Enables output of the RTC1HZ pin (1 Hz)"]
    _1 = 1,
}
impl From<Rcloe1> for bool {
    #[inline(always)]
    fn from(variant: Rcloe1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCLOE1` reader - RTC1HZ pin output control"]
pub type Rcloe1R = crate::BitReader<Rcloe1>;
impl Rcloe1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcloe1 {
        match self.bits {
            false => Rcloe1::_0,
            true => Rcloe1::_1,
        }
    }
    #[doc = "Disables output of the RTC1HZ pin (1 Hz)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcloe1::_0
    }
    #[doc = "Enables output of the RTC1HZ pin (1 Hz)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcloe1::_1
    }
}
#[doc = "Field `RCLOE1` writer - RTC1HZ pin output control"]
pub type Rcloe1W<'a, REG> = crate::BitWriter<'a, REG, Rcloe1>;
impl<'a, REG> Rcloe1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables output of the RTC1HZ pin (1 Hz)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcloe1::_0)
    }
    #[doc = "Enables output of the RTC1HZ pin (1 Hz)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcloe1::_1)
    }
}
#[doc = "Realtime clock operation control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtce {
    #[doc = "0: Stops counter operation"]
    _0 = 0,
    #[doc = "1: Starts counter operation"]
    _1 = 1,
}
impl From<Rtce> for bool {
    #[inline(always)]
    fn from(variant: Rtce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCE` reader - Realtime clock operation control"]
pub type RtceR = crate::BitReader<Rtce>;
impl RtceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtce {
        match self.bits {
            false => Rtce::_0,
            true => Rtce::_1,
        }
    }
    #[doc = "Stops counter operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtce::_0
    }
    #[doc = "Starts counter operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtce::_1
    }
}
#[doc = "Field `RTCE` writer - Realtime clock operation control"]
pub type RtceW<'a, REG> = crate::BitWriter<'a, REG, Rtce>;
impl<'a, REG> RtceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stops counter operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtce::_0)
    }
    #[doc = "Starts counter operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtce::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Fixed-cycle interrupt (RTC_ALM_OR_PRD) selection"]
    #[inline(always)]
    pub fn ct(&self) -> CtR {
        CtR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Selection of 12- or 24-hour system"]
    #[inline(always)]
    pub fn ampm(&self) -> AmpmR {
        AmpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selection of the operating clock for the realtime clock (RTCSCLK/RTCS128CLK)"]
    #[inline(always)]
    pub fn rtc128en(&self) -> Rtc128enR {
        Rtc128enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC1HZ pin output control"]
    #[inline(always)]
    pub fn rcloe1(&self) -> Rcloe1R {
        Rcloe1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Realtime clock operation control"]
    #[inline(always)]
    pub fn rtce(&self) -> RtceR {
        RtceR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Fixed-cycle interrupt (RTC_ALM_OR_PRD) selection"]
    #[inline(always)]
    pub fn ct(&mut self) -> CtW<Rtcc0Spec> {
        CtW::new(self, 0)
    }
    #[doc = "Bit 3 - Selection of 12- or 24-hour system"]
    #[inline(always)]
    pub fn ampm(&mut self) -> AmpmW<Rtcc0Spec> {
        AmpmW::new(self, 3)
    }
    #[doc = "Bit 4 - Selection of the operating clock for the realtime clock (RTCSCLK/RTCS128CLK)"]
    #[inline(always)]
    pub fn rtc128en(&mut self) -> Rtc128enW<Rtcc0Spec> {
        Rtc128enW::new(self, 4)
    }
    #[doc = "Bit 5 - RTC1HZ pin output control"]
    #[inline(always)]
    pub fn rcloe1(&mut self) -> Rcloe1W<Rtcc0Spec> {
        Rcloe1W::new(self, 5)
    }
    #[doc = "Bit 7 - Realtime clock operation control"]
    #[inline(always)]
    pub fn rtce(&mut self) -> RtceW<Rtcc0Spec> {
        RtceW::new(self, 7)
    }
}
#[doc = "Realtime Clock Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcc0Spec;
impl crate::RegisterSpec for Rtcc0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtcc0::R`](R) reader structure"]
impl crate::Readable for Rtcc0Spec {}
#[doc = "`write(|w| ..)` method takes [`rtcc0::W`](W) writer structure"]
impl crate::Writable for Rtcc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RTCC0 to value 0"]
impl crate::Resettable for Rtcc0Spec {
    const RESET_VALUE: u8 = 0;
}
