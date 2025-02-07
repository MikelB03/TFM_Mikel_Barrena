#[doc = "Register `DBGSTOPCR` reader"]
pub type R = crate::R<DbgstopcrSpec>;
#[doc = "Register `DBGSTOPCR` writer"]
pub type W = crate::W<DbgstopcrSpec>;
#[doc = "Mask bit for IWDT reset/interrupt in the OCD run mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopIwdt {
    #[doc = "0: Enable IWDT reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask IWDT reset/interrupt and stop WDT counter"]
    _1 = 1,
}
impl From<DbgstopIwdt> for bool {
    #[inline(always)]
    fn from(variant: DbgstopIwdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_IWDT` reader - Mask bit for IWDT reset/interrupt in the OCD run mode"]
pub type DbgstopIwdtR = crate::BitReader<DbgstopIwdt>;
impl DbgstopIwdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopIwdt {
        match self.bits {
            false => DbgstopIwdt::_0,
            true => DbgstopIwdt::_1,
        }
    }
    #[doc = "Enable IWDT reset/interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopIwdt::_0
    }
    #[doc = "Mask IWDT reset/interrupt and stop WDT counter"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopIwdt::_1
    }
}
#[doc = "Field `DBGSTOP_IWDT` writer - Mask bit for IWDT reset/interrupt in the OCD run mode"]
pub type DbgstopIwdtW<'a, REG> = crate::BitWriter<'a, REG, DbgstopIwdt>;
impl<'a, REG> DbgstopIwdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable IWDT reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopIwdt::_0)
    }
    #[doc = "Mask IWDT reset/interrupt and stop WDT counter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopIwdt::_1)
    }
}
#[doc = "Mask bit for WDT reset/interrupt in the OCD run mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopWdt {
    #[doc = "0: Enable WDT reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask WDT reset/interrupt and stop WDT counter"]
    _1 = 1,
}
impl From<DbgstopWdt> for bool {
    #[inline(always)]
    fn from(variant: DbgstopWdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_WDT` reader - Mask bit for WDT reset/interrupt in the OCD run mode"]
pub type DbgstopWdtR = crate::BitReader<DbgstopWdt>;
impl DbgstopWdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopWdt {
        match self.bits {
            false => DbgstopWdt::_0,
            true => DbgstopWdt::_1,
        }
    }
    #[doc = "Enable WDT reset/interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopWdt::_0
    }
    #[doc = "Mask WDT reset/interrupt and stop WDT counter"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopWdt::_1
    }
}
#[doc = "Field `DBGSTOP_WDT` writer - Mask bit for WDT reset/interrupt in the OCD run mode"]
pub type DbgstopWdtW<'a, REG> = crate::BitWriter<'a, REG, DbgstopWdt>;
impl<'a, REG> DbgstopWdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable WDT reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopWdt::_0)
    }
    #[doc = "Mask WDT reset/interrupt and stop WDT counter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopWdt::_1)
    }
}
#[doc = "Control bit for RTC and TAU operation in the OCD break mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopTim {
    #[doc = "0: Continue RTC and TAU operation"]
    _0 = 0,
    #[doc = "1: Stop RTC and TAU operation"]
    _1 = 1,
}
impl From<DbgstopTim> for bool {
    #[inline(always)]
    fn from(variant: DbgstopTim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_TIM` reader - Control bit for RTC and TAU operation in the OCD break mode"]
pub type DbgstopTimR = crate::BitReader<DbgstopTim>;
impl DbgstopTimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopTim {
        match self.bits {
            false => DbgstopTim::_0,
            true => DbgstopTim::_1,
        }
    }
    #[doc = "Continue RTC and TAU operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopTim::_0
    }
    #[doc = "Stop RTC and TAU operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopTim::_1
    }
}
#[doc = "Field `DBGSTOP_TIM` writer - Control bit for RTC and TAU operation in the OCD break mode"]
pub type DbgstopTimW<'a, REG> = crate::BitWriter<'a, REG, DbgstopTim>;
impl<'a, REG> DbgstopTimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue RTC and TAU operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopTim::_0)
    }
    #[doc = "Stop RTC and TAU operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopTim::_1)
    }
}
#[doc = "Control bit for SAU and IICA operation in the OCD break mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopSir {
    #[doc = "0: Continue SAU and IICA operation"]
    _0 = 0,
    #[doc = "1: Stop SAU and IICA operation"]
    _1 = 1,
}
impl From<DbgstopSir> for bool {
    #[inline(always)]
    fn from(variant: DbgstopSir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_SIR` reader - Control bit for SAU and IICA operation in the OCD break mode"]
pub type DbgstopSirR = crate::BitReader<DbgstopSir>;
impl DbgstopSirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopSir {
        match self.bits {
            false => DbgstopSir::_0,
            true => DbgstopSir::_1,
        }
    }
    #[doc = "Continue SAU and IICA operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopSir::_0
    }
    #[doc = "Stop SAU and IICA operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopSir::_1
    }
}
#[doc = "Field `DBGSTOP_SIR` writer - Control bit for SAU and IICA operation in the OCD break mode"]
pub type DbgstopSirW<'a, REG> = crate::BitWriter<'a, REG, DbgstopSir>;
impl<'a, REG> DbgstopSirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue SAU and IICA operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopSir::_0)
    }
    #[doc = "Stop SAU and IICA operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopSir::_1)
    }
}
#[doc = "Mask bit for LVD0 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopLvd0 {
    #[doc = "0: Enable LVD0 reset"]
    _0 = 0,
    #[doc = "1: Mask LVD0 reset"]
    _1 = 1,
}
impl From<DbgstopLvd0> for bool {
    #[inline(always)]
    fn from(variant: DbgstopLvd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_LVD0` reader - Mask bit for LVD0 reset"]
pub type DbgstopLvd0R = crate::BitReader<DbgstopLvd0>;
impl DbgstopLvd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopLvd0 {
        match self.bits {
            false => DbgstopLvd0::_0,
            true => DbgstopLvd0::_1,
        }
    }
    #[doc = "Enable LVD0 reset"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopLvd0::_0
    }
    #[doc = "Mask LVD0 reset"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopLvd0::_1
    }
}
#[doc = "Field `DBGSTOP_LVD0` writer - Mask bit for LVD0 reset"]
pub type DbgstopLvd0W<'a, REG> = crate::BitWriter<'a, REG, DbgstopLvd0>;
impl<'a, REG> DbgstopLvd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable LVD0 reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd0::_0)
    }
    #[doc = "Mask LVD0 reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd0::_1)
    }
}
#[doc = "Mask bit for LVD1 reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopLvd1 {
    #[doc = "0: Enable LVD1 reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask LVD1 reset/interrupt"]
    _1 = 1,
}
impl From<DbgstopLvd1> for bool {
    #[inline(always)]
    fn from(variant: DbgstopLvd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_LVD1` reader - Mask bit for LVD1 reset/interrupt"]
pub type DbgstopLvd1R = crate::BitReader<DbgstopLvd1>;
impl DbgstopLvd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopLvd1 {
        match self.bits {
            false => DbgstopLvd1::_0,
            true => DbgstopLvd1::_1,
        }
    }
    #[doc = "Enable LVD1 reset/interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopLvd1::_0
    }
    #[doc = "Mask LVD1 reset/interrupt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopLvd1::_1
    }
}
#[doc = "Field `DBGSTOP_LVD1` writer - Mask bit for LVD1 reset/interrupt"]
pub type DbgstopLvd1W<'a, REG> = crate::BitWriter<'a, REG, DbgstopLvd1>;
impl<'a, REG> DbgstopLvd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable LVD1 reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd1::_0)
    }
    #[doc = "Mask LVD1 reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd1::_1)
    }
}
#[doc = "Mask bit for LVD2 reset/interrpt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopLvd2 {
    #[doc = "0: Enable LVD2 reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask LVD2 reset/interrupt"]
    _1 = 1,
}
impl From<DbgstopLvd2> for bool {
    #[inline(always)]
    fn from(variant: DbgstopLvd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_LVD2` reader - Mask bit for LVD2 reset/interrpt"]
pub type DbgstopLvd2R = crate::BitReader<DbgstopLvd2>;
impl DbgstopLvd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopLvd2 {
        match self.bits {
            false => DbgstopLvd2::_0,
            true => DbgstopLvd2::_1,
        }
    }
    #[doc = "Enable LVD2 reset/interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopLvd2::_0
    }
    #[doc = "Mask LVD2 reset/interrupt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopLvd2::_1
    }
}
#[doc = "Field `DBGSTOP_LVD2` writer - Mask bit for LVD2 reset/interrpt"]
pub type DbgstopLvd2W<'a, REG> = crate::BitWriter<'a, REG, DbgstopLvd2>;
impl<'a, REG> DbgstopLvd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable LVD2 reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd2::_0)
    }
    #[doc = "Mask LVD2 reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd2::_1)
    }
}
#[doc = "Mask bit for RAM parity error reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopRper {
    #[doc = "0: Enable RAM parity error reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask RAM parity error reset/interrupt"]
    _1 = 1,
}
impl From<DbgstopRper> for bool {
    #[inline(always)]
    fn from(variant: DbgstopRper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_RPER` reader - Mask bit for RAM parity error reset/interrupt"]
pub type DbgstopRperR = crate::BitReader<DbgstopRper>;
impl DbgstopRperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopRper {
        match self.bits {
            false => DbgstopRper::_0,
            true => DbgstopRper::_1,
        }
    }
    #[doc = "Enable RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopRper::_0
    }
    #[doc = "Mask RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopRper::_1
    }
}
#[doc = "Field `DBGSTOP_RPER` writer - Mask bit for RAM parity error reset/interrupt"]
pub type DbgstopRperW<'a, REG> = crate::BitWriter<'a, REG, DbgstopRper>;
impl<'a, REG> DbgstopRperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopRper::_0)
    }
    #[doc = "Mask RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopRper::_1)
    }
}
#[doc = "Mask bit for RAM ECC error reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopReccr {
    #[doc = "0: Enable RAM ECC error reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask RAM ECC error reset/interrupt"]
    _1 = 1,
}
impl From<DbgstopReccr> for bool {
    #[inline(always)]
    fn from(variant: DbgstopReccr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGSTOP_RECCR` reader - Mask bit for RAM ECC error reset/interrupt"]
pub type DbgstopReccrR = crate::BitReader<DbgstopReccr>;
impl DbgstopReccrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopReccr {
        match self.bits {
            false => DbgstopReccr::_0,
            true => DbgstopReccr::_1,
        }
    }
    #[doc = "Enable RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopReccr::_0
    }
    #[doc = "Mask RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopReccr::_1
    }
}
#[doc = "Field `DBGSTOP_RECCR` writer - Mask bit for RAM ECC error reset/interrupt"]
pub type DbgstopReccrW<'a, REG> = crate::BitWriter<'a, REG, DbgstopReccr>;
impl<'a, REG> DbgstopReccrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopReccr::_0)
    }
    #[doc = "Mask RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopReccr::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Mask bit for IWDT reset/interrupt in the OCD run mode"]
    #[inline(always)]
    pub fn dbgstop_iwdt(&self) -> DbgstopIwdtR {
        DbgstopIwdtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for WDT reset/interrupt in the OCD run mode"]
    #[inline(always)]
    pub fn dbgstop_wdt(&self) -> DbgstopWdtR {
        DbgstopWdtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - Control bit for RTC and TAU operation in the OCD break mode"]
    #[inline(always)]
    pub fn dbgstop_tim(&self) -> DbgstopTimR {
        DbgstopTimR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Control bit for SAU and IICA operation in the OCD break mode"]
    #[inline(always)]
    pub fn dbgstop_sir(&self) -> DbgstopSirR {
        DbgstopSirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask bit for LVD0 reset"]
    #[inline(always)]
    pub fn dbgstop_lvd0(&self) -> DbgstopLvd0R {
        DbgstopLvd0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask bit for LVD1 reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_lvd1(&self) -> DbgstopLvd1R {
        DbgstopLvd1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mask bit for LVD2 reset/interrpt"]
    #[inline(always)]
    pub fn dbgstop_lvd2(&self) -> DbgstopLvd2R {
        DbgstopLvd2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Mask bit for RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_rper(&self) -> DbgstopRperR {
        DbgstopRperR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mask bit for RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_reccr(&self) -> DbgstopReccrR {
        DbgstopReccrR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for IWDT reset/interrupt in the OCD run mode"]
    #[inline(always)]
    pub fn dbgstop_iwdt(&mut self) -> DbgstopIwdtW<DbgstopcrSpec> {
        DbgstopIwdtW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for WDT reset/interrupt in the OCD run mode"]
    #[inline(always)]
    pub fn dbgstop_wdt(&mut self) -> DbgstopWdtW<DbgstopcrSpec> {
        DbgstopWdtW::new(self, 1)
    }
    #[doc = "Bit 14 - Control bit for RTC and TAU operation in the OCD break mode"]
    #[inline(always)]
    pub fn dbgstop_tim(&mut self) -> DbgstopTimW<DbgstopcrSpec> {
        DbgstopTimW::new(self, 14)
    }
    #[doc = "Bit 15 - Control bit for SAU and IICA operation in the OCD break mode"]
    #[inline(always)]
    pub fn dbgstop_sir(&mut self) -> DbgstopSirW<DbgstopcrSpec> {
        DbgstopSirW::new(self, 15)
    }
    #[doc = "Bit 16 - Mask bit for LVD0 reset"]
    #[inline(always)]
    pub fn dbgstop_lvd0(&mut self) -> DbgstopLvd0W<DbgstopcrSpec> {
        DbgstopLvd0W::new(self, 16)
    }
    #[doc = "Bit 17 - Mask bit for LVD1 reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_lvd1(&mut self) -> DbgstopLvd1W<DbgstopcrSpec> {
        DbgstopLvd1W::new(self, 17)
    }
    #[doc = "Bit 18 - Mask bit for LVD2 reset/interrpt"]
    #[inline(always)]
    pub fn dbgstop_lvd2(&mut self) -> DbgstopLvd2W<DbgstopcrSpec> {
        DbgstopLvd2W::new(self, 18)
    }
    #[doc = "Bit 24 - Mask bit for RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_rper(&mut self) -> DbgstopRperW<DbgstopcrSpec> {
        DbgstopRperW::new(self, 24)
    }
    #[doc = "Bit 25 - Mask bit for RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_reccr(&mut self) -> DbgstopReccrW<DbgstopcrSpec> {
        DbgstopReccrW::new(self, 25)
    }
}
#[doc = "Debug Stop Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgstopcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgstopcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgstopcrSpec;
impl crate::RegisterSpec for DbgstopcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgstopcr::R`](R) reader structure"]
impl crate::Readable for DbgstopcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgstopcr::W`](W) writer structure"]
impl crate::Writable for DbgstopcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGSTOPCR to value 0x03"]
impl crate::Resettable for DbgstopcrSpec {
    const RESET_VALUE: u32 = 0x03;
}
