#[doc = "Register `WUPEN1` reader"]
pub type R = crate::R<Wupen1Spec>;
#[doc = "Register `WUPEN1` writer"]
pub type W = crate::W<Wupen1Spec>;
#[doc = "IIC0_ENDI/IIC0_WUI Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iica0wupen {
    #[doc = "0: Software Standby/Snooze mode returns by IIC0_ENDI/IIC0_WUI interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze mode returns by IIC0_ENDI/IIC0_WUI interrupt enabled"]
    _1 = 1,
}
impl From<Iica0wupen> for bool {
    #[inline(always)]
    fn from(variant: Iica0wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICA0WUPEN` reader - IIC0_ENDI/IIC0_WUI Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Iica0wupenR = crate::BitReader<Iica0wupen>;
impl Iica0wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iica0wupen {
        match self.bits {
            false => Iica0wupen::_0,
            true => Iica0wupen::_1,
        }
    }
    #[doc = "Software Standby/Snooze mode returns by IIC0_ENDI/IIC0_WUI interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iica0wupen::_0
    }
    #[doc = "Software Standby/Snooze mode returns by IIC0_ENDI/IIC0_WUI interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iica0wupen::_1
    }
}
#[doc = "Field `IICA0WUPEN` writer - IIC0_ENDI/IIC0_WUI Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Iica0wupenW<'a, REG> = crate::BitWriter<'a, REG, Iica0wupen>;
impl<'a, REG> Iica0wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software Standby/Snooze mode returns by IIC0_ENDI/IIC0_WUI interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iica0wupen::_0)
    }
    #[doc = "Software Standby/Snooze mode returns by IIC0_ENDI/IIC0_WUI interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iica0wupen::_1)
    }
}
#[doc = "IIC1_ENDI/IIC1_WUI Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iica1wupen {
    #[doc = "0: Software Standby/Snooze mode returns by IIC1_ENDI/IIC1_WUI interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze mode returns by IIC1_ENDI/IIC1_WUI interrupt enabled"]
    _1 = 1,
}
impl From<Iica1wupen> for bool {
    #[inline(always)]
    fn from(variant: Iica1wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICA1WUPEN` reader - IIC1_ENDI/IIC1_WUI Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Iica1wupenR = crate::BitReader<Iica1wupen>;
impl Iica1wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iica1wupen {
        match self.bits {
            false => Iica1wupen::_0,
            true => Iica1wupen::_1,
        }
    }
    #[doc = "Software Standby/Snooze mode returns by IIC1_ENDI/IIC1_WUI interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iica1wupen::_0
    }
    #[doc = "Software Standby/Snooze mode returns by IIC1_ENDI/IIC1_WUI interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iica1wupen::_1
    }
}
#[doc = "Field `IICA1WUPEN` writer - IIC1_ENDI/IIC1_WUI Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Iica1wupenW<'a, REG> = crate::BitWriter<'a, REG, Iica1wupen>;
impl<'a, REG> Iica1wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software Standby/Snooze mode returns by IIC1_ENDI/IIC1_WUI interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iica1wupen::_0)
    }
    #[doc = "Software Standby/Snooze mode returns by IIC1_ENDI/IIC1_WUI interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iica1wupen::_1)
    }
}
#[doc = "UARTA_RX_ENDI0 Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uartarxendi0wupen {
    #[doc = "0: Software standby/Snooze mode returns by UARTA_RX_ENDI0 interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software standby/Snooze mode returns by UARTA_RX_ENDI0 interrupt enabled"]
    _1 = 1,
}
impl From<Uartarxendi0wupen> for bool {
    #[inline(always)]
    fn from(variant: Uartarxendi0wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTARXENDI0WUPEN` reader - UARTA_RX_ENDI0 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Uartarxendi0wupenR = crate::BitReader<Uartarxendi0wupen>;
impl Uartarxendi0wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uartarxendi0wupen {
        match self.bits {
            false => Uartarxendi0wupen::_0,
            true => Uartarxendi0wupen::_1,
        }
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ENDI0 interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uartarxendi0wupen::_0
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ENDI0 interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uartarxendi0wupen::_1
    }
}
#[doc = "Field `UARTARXENDI0WUPEN` writer - UARTA_RX_ENDI0 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Uartarxendi0wupenW<'a, REG> = crate::BitWriter<'a, REG, Uartarxendi0wupen>;
impl<'a, REG> Uartarxendi0wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ENDI0 interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uartarxendi0wupen::_0)
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ENDI0 interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uartarxendi0wupen::_1)
    }
}
#[doc = "UARTA_RX_ERI0 Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uartarxeri0wupen {
    #[doc = "0: Software standby/Snooze mode returns by UARTA_RX_ERI0 interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software standby/Snooze mode returns by UARTA_RX_ERI0 interrupt enabled"]
    _1 = 1,
}
impl From<Uartarxeri0wupen> for bool {
    #[inline(always)]
    fn from(variant: Uartarxeri0wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTARXERI0WUPEN` reader - UARTA_RX_ERI0 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Uartarxeri0wupenR = crate::BitReader<Uartarxeri0wupen>;
impl Uartarxeri0wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uartarxeri0wupen {
        match self.bits {
            false => Uartarxeri0wupen::_0,
            true => Uartarxeri0wupen::_1,
        }
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ERI0 interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uartarxeri0wupen::_0
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ERI0 interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uartarxeri0wupen::_1
    }
}
#[doc = "Field `UARTARXERI0WUPEN` writer - UARTA_RX_ERI0 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Uartarxeri0wupenW<'a, REG> = crate::BitWriter<'a, REG, Uartarxeri0wupen>;
impl<'a, REG> Uartarxeri0wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ERI0 interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uartarxeri0wupen::_0)
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ERI0 interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uartarxeri0wupen::_1)
    }
}
#[doc = "UARTA_RX_ENDI1 Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uartarxendi1wupen {
    #[doc = "0: Software standby/Snooze mode returns by UARTA_RX_ENDI1 interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software standby/Snooze mode returns by UARTA_RX_ENDI1 interrupt enabled"]
    _1 = 1,
}
impl From<Uartarxendi1wupen> for bool {
    #[inline(always)]
    fn from(variant: Uartarxendi1wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTARXENDI1WUPEN` reader - UARTA_RX_ENDI1 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Uartarxendi1wupenR = crate::BitReader<Uartarxendi1wupen>;
impl Uartarxendi1wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uartarxendi1wupen {
        match self.bits {
            false => Uartarxendi1wupen::_0,
            true => Uartarxendi1wupen::_1,
        }
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ENDI1 interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uartarxendi1wupen::_0
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ENDI1 interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uartarxendi1wupen::_1
    }
}
#[doc = "Field `UARTARXENDI1WUPEN` writer - UARTA_RX_ENDI1 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Uartarxendi1wupenW<'a, REG> = crate::BitWriter<'a, REG, Uartarxendi1wupen>;
impl<'a, REG> Uartarxendi1wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ENDI1 interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uartarxendi1wupen::_0)
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ENDI1 interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uartarxendi1wupen::_1)
    }
}
#[doc = "UARTA_RX_ERI1 Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uartarxeri1wupen {
    #[doc = "0: Software standby/Snooze mode returns by UARTA_RX_ERI1 interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software standby/Snooze mode returns by UARTA_RX_ERI1 interrupt enabled"]
    _1 = 1,
}
impl From<Uartarxeri1wupen> for bool {
    #[inline(always)]
    fn from(variant: Uartarxeri1wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTARXERI1WUPEN` reader - UARTA_RX_ERI1 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Uartarxeri1wupenR = crate::BitReader<Uartarxeri1wupen>;
impl Uartarxeri1wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uartarxeri1wupen {
        match self.bits {
            false => Uartarxeri1wupen::_0,
            true => Uartarxeri1wupen::_1,
        }
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ERI1 interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uartarxeri1wupen::_0
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ERI1 interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uartarxeri1wupen::_1
    }
}
#[doc = "Field `UARTARXERI1WUPEN` writer - UARTA_RX_ERI1 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Uartarxeri1wupenW<'a, REG> = crate::BitWriter<'a, REG, Uartarxeri1wupen>;
impl<'a, REG> Uartarxeri1wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ERI1 interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uartarxeri1wupen::_0)
    }
    #[doc = "Software standby/Snooze mode returns by UARTA_RX_ERI1 interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uartarxeri1wupen::_1)
    }
}
#[doc = "COMP_DET0 Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compdet0wupen {
    #[doc = "0: Software standby/Snooze mode returns by COMP_DET0 interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software standby/Snooze mode returns by COMP_DET0 interrupt enabled"]
    _1 = 1,
}
impl From<Compdet0wupen> for bool {
    #[inline(always)]
    fn from(variant: Compdet0wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPDET0WUPEN` reader - COMP_DET0 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Compdet0wupenR = crate::BitReader<Compdet0wupen>;
impl Compdet0wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compdet0wupen {
        match self.bits {
            false => Compdet0wupen::_0,
            true => Compdet0wupen::_1,
        }
    }
    #[doc = "Software standby/Snooze mode returns by COMP_DET0 interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Compdet0wupen::_0
    }
    #[doc = "Software standby/Snooze mode returns by COMP_DET0 interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Compdet0wupen::_1
    }
}
#[doc = "Field `COMPDET0WUPEN` writer - COMP_DET0 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Compdet0wupenW<'a, REG> = crate::BitWriter<'a, REG, Compdet0wupen>;
impl<'a, REG> Compdet0wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software standby/Snooze mode returns by COMP_DET0 interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Compdet0wupen::_0)
    }
    #[doc = "Software standby/Snooze mode returns by COMP_DET0 interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Compdet0wupen::_1)
    }
}
#[doc = "COMP_DET1 Interrupt Software Standby/Snooze Mode Returns Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compdet1wupen {
    #[doc = "0: Software standby/Snooze mode returns by COMP_DET1 interrupt disabled"]
    _0 = 0,
    #[doc = "1: Software standby/Snooze mode returns by COMP_DET1 interrupt enabled"]
    _1 = 1,
}
impl From<Compdet1wupen> for bool {
    #[inline(always)]
    fn from(variant: Compdet1wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPDET1WUPEN` reader - COMP_DET1 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Compdet1wupenR = crate::BitReader<Compdet1wupen>;
impl Compdet1wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compdet1wupen {
        match self.bits {
            false => Compdet1wupen::_0,
            true => Compdet1wupen::_1,
        }
    }
    #[doc = "Software standby/Snooze mode returns by COMP_DET1 interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Compdet1wupen::_0
    }
    #[doc = "Software standby/Snooze mode returns by COMP_DET1 interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Compdet1wupen::_1
    }
}
#[doc = "Field `COMPDET1WUPEN` writer - COMP_DET1 Interrupt Software Standby/Snooze Mode Returns Enable"]
pub type Compdet1wupenW<'a, REG> = crate::BitWriter<'a, REG, Compdet1wupen>;
impl<'a, REG> Compdet1wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software standby/Snooze mode returns by COMP_DET1 interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Compdet1wupen::_0)
    }
    #[doc = "Software standby/Snooze mode returns by COMP_DET1 interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Compdet1wupen::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IIC0_ENDI/IIC0_WUI Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn iica0wupen(&self) -> Iica0wupenR {
        Iica0wupenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IIC1_ENDI/IIC1_WUI Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn iica1wupen(&self) -> Iica1wupenR {
        Iica1wupenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UARTA_RX_ENDI0 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn uartarxendi0wupen(&self) -> Uartarxendi0wupenR {
        Uartarxendi0wupenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UARTA_RX_ERI0 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn uartarxeri0wupen(&self) -> Uartarxeri0wupenR {
        Uartarxeri0wupenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UARTA_RX_ENDI1 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn uartarxendi1wupen(&self) -> Uartarxendi1wupenR {
        Uartarxendi1wupenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UARTA_RX_ERI1 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn uartarxeri1wupen(&self) -> Uartarxeri1wupenR {
        Uartarxeri1wupenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - COMP_DET0 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn compdet0wupen(&self) -> Compdet0wupenR {
        Compdet0wupenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - COMP_DET1 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn compdet1wupen(&self) -> Compdet1wupenR {
        Compdet1wupenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IIC0_ENDI/IIC0_WUI Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn iica0wupen(&mut self) -> Iica0wupenW<Wupen1Spec> {
        Iica0wupenW::new(self, 0)
    }
    #[doc = "Bit 1 - IIC1_ENDI/IIC1_WUI Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn iica1wupen(&mut self) -> Iica1wupenW<Wupen1Spec> {
        Iica1wupenW::new(self, 1)
    }
    #[doc = "Bit 2 - UARTA_RX_ENDI0 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn uartarxendi0wupen(&mut self) -> Uartarxendi0wupenW<Wupen1Spec> {
        Uartarxendi0wupenW::new(self, 2)
    }
    #[doc = "Bit 3 - UARTA_RX_ERI0 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn uartarxeri0wupen(&mut self) -> Uartarxeri0wupenW<Wupen1Spec> {
        Uartarxeri0wupenW::new(self, 3)
    }
    #[doc = "Bit 4 - UARTA_RX_ENDI1 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn uartarxendi1wupen(&mut self) -> Uartarxendi1wupenW<Wupen1Spec> {
        Uartarxendi1wupenW::new(self, 4)
    }
    #[doc = "Bit 5 - UARTA_RX_ERI1 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn uartarxeri1wupen(&mut self) -> Uartarxeri1wupenW<Wupen1Spec> {
        Uartarxeri1wupenW::new(self, 5)
    }
    #[doc = "Bit 6 - COMP_DET0 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn compdet0wupen(&mut self) -> Compdet0wupenW<Wupen1Spec> {
        Compdet0wupenW::new(self, 6)
    }
    #[doc = "Bit 7 - COMP_DET1 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn compdet1wupen(&mut self) -> Compdet1wupenW<Wupen1Spec> {
        Compdet1wupenW::new(self, 7)
    }
}
#[doc = "Wake Up Interrupt Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`wupen1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wupen1Spec;
impl crate::RegisterSpec for Wupen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wupen1::R`](R) reader structure"]
impl crate::Readable for Wupen1Spec {}
#[doc = "`write(|w| ..)` method takes [`wupen1::W`](W) writer structure"]
impl crate::Writable for Wupen1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUPEN1 to value 0"]
impl crate::Resettable for Wupen1Spec {
    const RESET_VALUE: u32 = 0;
}
