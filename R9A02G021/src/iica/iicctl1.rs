#[doc = "Register `IICCTL%s1` reader"]
pub type R = crate::R<Iicctl1Spec>;
#[doc = "Register `IICCTL%s1` writer"]
pub type W = crate::W<Iicctl1Spec>;
#[doc = "IICA operation clock (fMCK)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prs {
    #[doc = "0: Selects PCLKB (1 MHz ≤ PCLKB ≤ 20 MHz)"]
    _0 = 0,
    #[doc = "1: Selects PCLKB/2 (20 MHz < PCLKB ≤ 32 MHz)"]
    _1 = 1,
}
impl From<Prs> for bool {
    #[inline(always)]
    fn from(variant: Prs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRS` reader - IICA operation clock (fMCK)"]
pub type PrsR = crate::BitReader<Prs>;
impl PrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prs {
        match self.bits {
            false => Prs::_0,
            true => Prs::_1,
        }
    }
    #[doc = "Selects PCLKB (1 MHz ≤ PCLKB ≤ 20 MHz)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prs::_0
    }
    #[doc = "Selects PCLKB/2 (20 MHz < PCLKB ≤ 32 MHz)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prs::_1
    }
}
#[doc = "Field `PRS` writer - IICA operation clock (fMCK)"]
pub type PrsW<'a, REG> = crate::BitWriter<'a, REG, Prs>;
impl<'a, REG> PrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects PCLKB (1 MHz ≤ PCLKB ≤ 20 MHz)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Prs::_0)
    }
    #[doc = "Selects PCLKB/2 (20 MHz < PCLKB ≤ 32 MHz)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Prs::_1)
    }
}
#[doc = "Digital filter operation control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfc {
    #[doc = "0: Digital filter off"]
    _0 = 0,
    #[doc = "1: Digital filter on"]
    _1 = 1,
}
impl From<Dfc> for bool {
    #[inline(always)]
    fn from(variant: Dfc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFC` reader - Digital filter operation control"]
pub type DfcR = crate::BitReader<Dfc>;
impl DfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfc {
        match self.bits {
            false => Dfc::_0,
            true => Dfc::_1,
        }
    }
    #[doc = "Digital filter off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dfc::_0
    }
    #[doc = "Digital filter on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dfc::_1
    }
}
#[doc = "Field `DFC` writer - Digital filter operation control"]
pub type DfcW<'a, REG> = crate::BitWriter<'a, REG, Dfc>;
impl<'a, REG> DfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital filter off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfc::_0)
    }
    #[doc = "Digital filter on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfc::_1)
    }
}
#[doc = "Operation mode switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smc {
    #[doc = "0: Operates in standard mode (fastest transfer rate: 100 kbps)"]
    _0 = 0,
    #[doc = "1: Operates in fast mode (fastest transfer rate: 400 kbps) or fast mode plus (fastest transfer rate: 1 Mbps)"]
    _1 = 1,
}
impl From<Smc> for bool {
    #[inline(always)]
    fn from(variant: Smc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMC` reader - Operation mode switching"]
pub type SmcR = crate::BitReader<Smc>;
impl SmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smc {
        match self.bits {
            false => Smc::_0,
            true => Smc::_1,
        }
    }
    #[doc = "Operates in standard mode (fastest transfer rate: 100 kbps)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Smc::_0
    }
    #[doc = "Operates in fast mode (fastest transfer rate: 400 kbps) or fast mode plus (fastest transfer rate: 1 Mbps)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Smc::_1
    }
}
#[doc = "Field `SMC` writer - Operation mode switching"]
pub type SmcW<'a, REG> = crate::BitWriter<'a, REG, Smc>;
impl<'a, REG> SmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operates in standard mode (fastest transfer rate: 100 kbps)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::_0)
    }
    #[doc = "Operates in fast mode (fastest transfer rate: 400 kbps) or fast mode plus (fastest transfer rate: 1 Mbps)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::_1)
    }
}
#[doc = "Detection of SDAAn pin level (valid only when IICCTLn0.IICE = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dad {
    #[doc = "0: The SDAAn pin was detected at low level"]
    _0 = 0,
    #[doc = "1: The SDAAn pin was detected at high level"]
    _1 = 1,
}
impl From<Dad> for bool {
    #[inline(always)]
    fn from(variant: Dad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAD` reader - Detection of SDAAn pin level (valid only when IICCTLn0.IICE = 1)"]
pub type DadR = crate::BitReader<Dad>;
impl DadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dad {
        match self.bits {
            false => Dad::_0,
            true => Dad::_1,
        }
    }
    #[doc = "The SDAAn pin was detected at low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dad::_0
    }
    #[doc = "The SDAAn pin was detected at high level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dad::_1
    }
}
#[doc = "Detection of SCLAn pin level (valid only when IICCTLn0.IICE = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cld {
    #[doc = "0: The SCLAn pin was detected at low level"]
    _0 = 0,
    #[doc = "1: The SCLAn pin was detected at high level"]
    _1 = 1,
}
impl From<Cld> for bool {
    #[inline(always)]
    fn from(variant: Cld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLD` reader - Detection of SCLAn pin level (valid only when IICCTLn0.IICE = 1)"]
pub type CldR = crate::BitReader<Cld>;
impl CldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cld {
        match self.bits {
            false => Cld::_0,
            true => Cld::_1,
        }
    }
    #[doc = "The SCLAn pin was detected at low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cld::_0
    }
    #[doc = "The SCLAn pin was detected at high level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cld::_1
    }
}
#[doc = "Address match disabling flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svadis {
    #[doc = "0: Disables the all address match function"]
    _0 = 0,
    #[doc = "1: Enables the all address match function"]
    _1 = 1,
}
impl From<Svadis> for bool {
    #[inline(always)]
    fn from(variant: Svadis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVADIS` reader - Address match disabling flag"]
pub type SvadisR = crate::BitReader<Svadis>;
impl SvadisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svadis {
        match self.bits {
            false => Svadis::_0,
            true => Svadis::_1,
        }
    }
    #[doc = "Disables the all address match function"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Svadis::_0
    }
    #[doc = "Enables the all address match function"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Svadis::_1
    }
}
#[doc = "Field `SVADIS` writer - Address match disabling flag"]
pub type SvadisW<'a, REG> = crate::BitWriter<'a, REG, Svadis>;
impl<'a, REG> SvadisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the all address match function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Svadis::_0)
    }
    #[doc = "Enables the all address match function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Svadis::_1)
    }
}
#[doc = "Control of address match wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wup {
    #[doc = "0: Stops operation of address match wakeup function in Software Standby mode"]
    _0 = 0,
    #[doc = "1: Enables operation of address match wakeup function in Software Standby mode"]
    _1 = 1,
}
impl From<Wup> for bool {
    #[inline(always)]
    fn from(variant: Wup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUP` reader - Control of address match wakeup"]
pub type WupR = crate::BitReader<Wup>;
impl WupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wup {
        match self.bits {
            false => Wup::_0,
            true => Wup::_1,
        }
    }
    #[doc = "Stops operation of address match wakeup function in Software Standby mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wup::_0
    }
    #[doc = "Enables operation of address match wakeup function in Software Standby mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wup::_1
    }
}
#[doc = "Field `WUP` writer - Control of address match wakeup"]
pub type WupW<'a, REG> = crate::BitWriter<'a, REG, Wup>;
impl<'a, REG> WupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stops operation of address match wakeup function in Software Standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wup::_0)
    }
    #[doc = "Enables operation of address match wakeup function in Software Standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wup::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IICA operation clock (fMCK)"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Digital filter operation control"]
    #[inline(always)]
    pub fn dfc(&self) -> DfcR {
        DfcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Operation mode switching"]
    #[inline(always)]
    pub fn smc(&self) -> SmcR {
        SmcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Detection of SDAAn pin level (valid only when IICCTLn0.IICE = 1)"]
    #[inline(always)]
    pub fn dad(&self) -> DadR {
        DadR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Detection of SCLAn pin level (valid only when IICCTLn0.IICE = 1)"]
    #[inline(always)]
    pub fn cld(&self) -> CldR {
        CldR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Address match disabling flag"]
    #[inline(always)]
    pub fn svadis(&self) -> SvadisR {
        SvadisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Control of address match wakeup"]
    #[inline(always)]
    pub fn wup(&self) -> WupR {
        WupR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IICA operation clock (fMCK)"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<Iicctl1Spec> {
        PrsW::new(self, 0)
    }
    #[doc = "Bit 2 - Digital filter operation control"]
    #[inline(always)]
    pub fn dfc(&mut self) -> DfcW<Iicctl1Spec> {
        DfcW::new(self, 2)
    }
    #[doc = "Bit 3 - Operation mode switching"]
    #[inline(always)]
    pub fn smc(&mut self) -> SmcW<Iicctl1Spec> {
        SmcW::new(self, 3)
    }
    #[doc = "Bit 6 - Address match disabling flag"]
    #[inline(always)]
    pub fn svadis(&mut self) -> SvadisW<Iicctl1Spec> {
        SvadisW::new(self, 6)
    }
    #[doc = "Bit 7 - Control of address match wakeup"]
    #[inline(always)]
    pub fn wup(&mut self) -> WupW<Iicctl1Spec> {
        WupW::new(self, 7)
    }
}
#[doc = "IICA Control Register n1\n\nYou can [`read`](crate::Reg::read) this register and get [`iicctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iicctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iicctl1Spec;
impl crate::RegisterSpec for Iicctl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iicctl1::R`](R) reader structure"]
impl crate::Readable for Iicctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`iicctl1::W`](W) writer structure"]
impl crate::Writable for Iicctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IICCTL%s1 to value 0"]
impl crate::Resettable for Iicctl1Spec {
    const RESET_VALUE: u8 = 0;
}
