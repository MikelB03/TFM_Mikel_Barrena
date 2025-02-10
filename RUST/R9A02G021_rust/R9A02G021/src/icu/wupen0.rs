#[doc = "Register `WUPEN0` reader"]
pub type R = crate::R<Wupen0Spec>;
#[doc = "Register `WUPEN0` writer"]
pub type W = crate::W<Wupen0Spec>;
#[doc = "IRQn (n = 0 to 7) Interrupt Software Standby/Snooze Mode Returns Enable, whereas IRQn corresponds to IRQWUPEN\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irqwupen {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt enabled"]
    _1 = 1,
}
impl From<Irqwupen> for u8 {
    #[inline(always)]
    fn from(variant: Irqwupen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irqwupen {
    type Ux = u8;
}
impl crate::IsEnum for Irqwupen {}
#[doc = "Field `IRQWUPEN` reader - IRQn (n = 0 to 7) Interrupt Software Standby/Snooze Mode Returns Enable, whereas IRQn corresponds to IRQWUPEN\\[n\\]"]
pub type IrqwupenR = crate::FieldReader<Irqwupen>;
impl IrqwupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Irqwupen> {
        match self.bits {
            0 => Some(Irqwupen::_0),
            1 => Some(Irqwupen::_1),
            _ => None,
        }
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen::_0
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen::_1
    }
}
#[doc = "Field `IRQWUPEN` writer - IRQn (n = 0 to 7) Interrupt Software Standby/Snooze Mode Returns Enable, whereas IRQn corresponds to IRQWUPEN\\[n\\]"]
pub type IrqwupenW<'a, REG> = crate::FieldWriter<'a, REG, 8, Irqwupen>;
impl<'a, REG> IrqwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen::_1)
    }
}
#[doc = "IWDT Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtwupen {
    #[doc = "0: Software Standby/Snooze Mode returns by IWDT interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IWDT interrupt enabled"]
    _1 = 1,
}
impl From<Iwdtwupen> for bool {
    #[inline(always)]
    fn from(variant: Iwdtwupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDTWUPEN` reader - IWDT Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type IwdtwupenR = crate::BitReader<Iwdtwupen>;
impl IwdtwupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtwupen {
        match self.bits {
            false => Iwdtwupen::_0,
            true => Iwdtwupen::_1,
        }
    }
    #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtwupen::_0
    }
    #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtwupen::_1
    }
}
#[doc = "Field `IWDTWUPEN` writer - IWDT Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type IwdtwupenW<'a, REG> = crate::BitWriter<'a, REG, Iwdtwupen>;
impl<'a, REG> IwdtwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtwupen::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtwupen::_1)
    }
}
#[doc = "Key Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keywupen {
    #[doc = "0: Software Standby/Snooze Mode returns by Key interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by Key interrupt enabled"]
    _1 = 1,
}
impl From<Keywupen> for bool {
    #[inline(always)]
    fn from(variant: Keywupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYWUPEN` reader - Key Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type KeywupenR = crate::BitReader<Keywupen>;
impl KeywupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keywupen {
        match self.bits {
            false => Keywupen::_0,
            true => Keywupen::_1,
        }
    }
    #[doc = "Software Standby/Snooze Mode returns by Key interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Keywupen::_0
    }
    #[doc = "Software Standby/Snooze Mode returns by Key interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Keywupen::_1
    }
}
#[doc = "Field `KEYWUPEN` writer - Key Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type KeywupenW<'a, REG> = crate::BitWriter<'a, REG, Keywupen>;
impl<'a, REG> KeywupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software Standby/Snooze Mode returns by Key interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Keywupen::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by Key interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Keywupen::_1)
    }
}
#[doc = "LVD1 Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1wupen {
    #[doc = "0: Software Standby/Snooze Mode returns by LVD1 interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by LVD1 interrupt enabled"]
    _1 = 1,
}
impl From<Lvd1wupen> for bool {
    #[inline(always)]
    fn from(variant: Lvd1wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD1WUPEN` reader - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Lvd1wupenR = crate::BitReader<Lvd1wupen>;
impl Lvd1wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1wupen {
        match self.bits {
            false => Lvd1wupen::_0,
            true => Lvd1wupen::_1,
        }
    }
    #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1wupen::_0
    }
    #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1wupen::_1
    }
}
#[doc = "Field `LVD1WUPEN` writer - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Lvd1wupenW<'a, REG> = crate::BitWriter<'a, REG, Lvd1wupen>;
impl<'a, REG> Lvd1wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1wupen::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1wupen::_1)
    }
}
#[doc = "LVD2 Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2wupen {
    #[doc = "0: Software Standby/Snooze Mode returns by LVD2 interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by LVD2 interrupt enabled"]
    _1 = 1,
}
impl From<Lvd2wupen> for bool {
    #[inline(always)]
    fn from(variant: Lvd2wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD2WUPEN` reader - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Lvd2wupenR = crate::BitReader<Lvd2wupen>;
impl Lvd2wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2wupen {
        match self.bits {
            false => Lvd2wupen::_0,
            true => Lvd2wupen::_1,
        }
    }
    #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2wupen::_0
    }
    #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2wupen::_1
    }
}
#[doc = "Field `LVD2WUPEN` writer - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Lvd2wupenW<'a, REG> = crate::BitWriter<'a, REG, Lvd2wupen>;
impl<'a, REG> Lvd2wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2wupen::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2wupen::_1)
    }
}
#[doc = "RTC Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcwupen {
    #[doc = "0: Software Standby/Snooze Mode returns by RTC interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by RTC interrupt enabled"]
    _1 = 1,
}
impl From<Rtcwupen> for bool {
    #[inline(always)]
    fn from(variant: Rtcwupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCWUPEN` reader - RTC Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type RtcwupenR = crate::BitReader<Rtcwupen>;
impl RtcwupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcwupen {
        match self.bits {
            false => Rtcwupen::_0,
            true => Rtcwupen::_1,
        }
    }
    #[doc = "Software Standby/Snooze Mode returns by RTC interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcwupen::_0
    }
    #[doc = "Software Standby/Snooze Mode returns by RTC interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcwupen::_1
    }
}
#[doc = "Field `RTCWUPEN` writer - RTC Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type RtcwupenW<'a, REG> = crate::BitWriter<'a, REG, Rtcwupen>;
impl<'a, REG> RtcwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software Standby/Snooze Mode returns by RTC interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcwupen::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by RTC interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcwupen::_1)
    }
}
#[doc = "TML32 Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tml32wupen {
    #[doc = "0: Software Standby/Snooze Mode returns by TML32 interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by TML32 interrupt enabled"]
    _1 = 1,
}
impl From<Tml32wupen> for bool {
    #[inline(always)]
    fn from(variant: Tml32wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TML32WUPEN` reader - TML32 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Tml32wupenR = crate::BitReader<Tml32wupen>;
impl Tml32wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tml32wupen {
        match self.bits {
            false => Tml32wupen::_0,
            true => Tml32wupen::_1,
        }
    }
    #[doc = "Software Standby/Snooze Mode returns by TML32 interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tml32wupen::_0
    }
    #[doc = "Software Standby/Snooze Mode returns by TML32 interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tml32wupen::_1
    }
}
#[doc = "Field `TML32WUPEN` writer - TML32 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Tml32wupenW<'a, REG> = crate::BitWriter<'a, REG, Tml32wupen>;
impl<'a, REG> Tml32wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software Standby/Snooze Mode returns by TML32 interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tml32wupen::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by TML32 interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tml32wupen::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - IRQn (n = 0 to 7) Interrupt Software Standby/Snooze Mode Returns Enable, whereas IRQn corresponds to IRQWUPEN\\[n\\]"]
    #[inline(always)]
    pub fn irqwupen(&self) -> IrqwupenR {
        IrqwupenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - IWDT Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn iwdtwupen(&self) -> IwdtwupenR {
        IwdtwupenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Key Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn keywupen(&self) -> KeywupenR {
        KeywupenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn lvd1wupen(&self) -> Lvd1wupenR {
        Lvd1wupenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn lvd2wupen(&self) -> Lvd2wupenR {
        Lvd2wupenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn rtcwupen(&self) -> RtcwupenR {
        RtcwupenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - TML32 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn tml32wupen(&self) -> Tml32wupenR {
        Tml32wupenR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - IRQn (n = 0 to 7) Interrupt Software Standby/Snooze Mode Returns Enable, whereas IRQn corresponds to IRQWUPEN\\[n\\]"]
    #[inline(always)]
    pub fn irqwupen(&mut self) -> IrqwupenW<Wupen0Spec> {
        IrqwupenW::new(self, 0)
    }
    #[doc = "Bit 16 - IWDT Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn iwdtwupen(&mut self) -> IwdtwupenW<Wupen0Spec> {
        IwdtwupenW::new(self, 16)
    }
    #[doc = "Bit 17 - Key Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn keywupen(&mut self) -> KeywupenW<Wupen0Spec> {
        KeywupenW::new(self, 17)
    }
    #[doc = "Bit 18 - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn lvd1wupen(&mut self) -> Lvd1wupenW<Wupen0Spec> {
        Lvd1wupenW::new(self, 18)
    }
    #[doc = "Bit 19 - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn lvd2wupen(&mut self) -> Lvd2wupenW<Wupen0Spec> {
        Lvd2wupenW::new(self, 19)
    }
    #[doc = "Bit 24 - RTC Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn rtcwupen(&mut self) -> RtcwupenW<Wupen0Spec> {
        RtcwupenW::new(self, 24)
    }
    #[doc = "Bit 26 - TML32 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn tml32wupen(&mut self) -> Tml32wupenW<Wupen0Spec> {
        Tml32wupenW::new(self, 26)
    }
}
#[doc = "Wake Up Interrupt Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wupen0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wupen0Spec;
impl crate::RegisterSpec for Wupen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wupen0::R`](R) reader structure"]
impl crate::Readable for Wupen0Spec {}
#[doc = "`write(|w| ..)` method takes [`wupen0::W`](W) writer structure"]
impl crate::Writable for Wupen0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUPEN0 to value 0"]
impl crate::Resettable for Wupen0Spec {
    const RESET_VALUE: u32 = 0;
}
