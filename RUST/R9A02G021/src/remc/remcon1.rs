#[doc = "Register `REMCON1` reader"]
pub type R = crate::R<Remcon1Spec>;
#[doc = "Register `REMCON1` writer"]
pub type W = crate::W<Remcon1Spec>;
#[doc = "Receive Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Typ {
    #[doc = "0: Format A"]
    _00 = 0,
    #[doc = "1: Format B"]
    _01 = 1,
    #[doc = "2: Format C"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Typ> for u8 {
    #[inline(always)]
    fn from(variant: Typ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Typ {
    type Ux = u8;
}
impl crate::IsEnum for Typ {}
#[doc = "Field `TYP` reader - Receive Mode Select"]
pub type TypR = crate::FieldReader<Typ>;
impl TypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Typ {
        match self.bits {
            0 => Typ::_00,
            1 => Typ::_01,
            2 => Typ::_10,
            3 => Typ::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Format A"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Typ::_00
    }
    #[doc = "Format B"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Typ::_01
    }
    #[doc = "Format C"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Typ::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Typ::_11
    }
}
#[doc = "Field `TYP` writer - Receive Mode Select"]
pub type TypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Typ, crate::Safe>;
impl<'a, REG> TypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Format A"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Typ::_00)
    }
    #[doc = "Format B"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Typ::_01)
    }
    #[doc = "Format C"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Typ::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Typ::_11)
    }
}
#[doc = "Remote Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Operation disabled"]
    _0 = 0,
    #[doc = "1: Operation enabled"]
    _1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Remote Control"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::_0,
            true => En::_1,
        }
    }
    #[doc = "Operation disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En::_0
    }
    #[doc = "Operation enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En::_1
    }
}
#[doc = "Field `EN` writer - Remote Control"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En::_0)
    }
    #[doc = "Operation enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En::_1)
    }
}
#[doc = "Interrupt Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intmd {
    #[doc = "0: Normal interrupt mode (OR condition)"]
    _0 = 0,
    #[doc = "1: Sequential interrupt mode (AND condition)"]
    _1 = 1,
}
impl From<Intmd> for bool {
    #[inline(always)]
    fn from(variant: Intmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTMD` reader - Interrupt Mode Select"]
pub type IntmdR = crate::BitReader<Intmd>;
impl IntmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intmd {
        match self.bits {
            false => Intmd::_0,
            true => Intmd::_1,
        }
    }
    #[doc = "Normal interrupt mode (OR condition)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Intmd::_0
    }
    #[doc = "Sequential interrupt mode (AND condition)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Intmd::_1
    }
}
#[doc = "Field `INTMD` writer - Interrupt Mode Select"]
pub type IntmdW<'a, REG> = crate::BitWriter<'a, REG, Intmd>;
impl<'a, REG> IntmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal interrupt mode (OR condition)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Intmd::_0)
    }
    #[doc = "Sequential interrupt mode (AND condition)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Intmd::_1)
    }
}
#[doc = "Operating Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csrc {
    #[doc = "0: REMCLCLK/REMCSCLK"]
    _0 = 0,
    #[doc = "1: Timer interrupt (TAU0_ENDI6) Use channel 6 of timer array unit in the interval timer mode"]
    _1 = 1,
}
impl From<Csrc> for bool {
    #[inline(always)]
    fn from(variant: Csrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSRC` reader - Operating Clock Select"]
pub type CsrcR = crate::BitReader<Csrc>;
impl CsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csrc {
        match self.bits {
            false => Csrc::_0,
            true => Csrc::_1,
        }
    }
    #[doc = "REMCLCLK/REMCSCLK"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Csrc::_0
    }
    #[doc = "Timer interrupt (TAU0_ENDI6) Use channel 6 of timer array unit in the interval timer mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Csrc::_1
    }
}
#[doc = "Field `CSRC` writer - Operating Clock Select"]
pub type CsrcW<'a, REG> = crate::BitWriter<'a, REG, Csrc>;
impl<'a, REG> CsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "REMCLCLK/REMCSCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Csrc::_0)
    }
    #[doc = "Timer interrupt (TAU0_ENDI6) Use channel 6 of timer array unit in the interval timer mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Csrc::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive Mode Select"]
    #[inline(always)]
    pub fn typ(&self) -> TypR {
        TypR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Remote Control"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mode Select"]
    #[inline(always)]
    pub fn intmd(&self) -> IntmdR {
        IntmdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Operating Clock Select"]
    #[inline(always)]
    pub fn csrc(&self) -> CsrcR {
        CsrcR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Mode Select"]
    #[inline(always)]
    pub fn typ(&mut self) -> TypW<Remcon1Spec> {
        TypW::new(self, 0)
    }
    #[doc = "Bit 2 - Remote Control"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Remcon1Spec> {
        EnW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Mode Select"]
    #[inline(always)]
    pub fn intmd(&mut self) -> IntmdW<Remcon1Spec> {
        IntmdW::new(self, 3)
    }
    #[doc = "Bit 5 - Operating Clock Select"]
    #[inline(always)]
    pub fn csrc(&mut self) -> CsrcW<Remcon1Spec> {
        CsrcW::new(self, 5)
    }
}
#[doc = "Function Select Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`remcon1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remcon1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remcon1Spec;
impl crate::RegisterSpec for Remcon1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`remcon1::R`](R) reader structure"]
impl crate::Readable for Remcon1Spec {}
#[doc = "`write(|w| ..)` method takes [`remcon1::W`](W) writer structure"]
impl crate::Writable for Remcon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets REMCON1 to value 0"]
impl crate::Resettable for Remcon1Spec {
    const RESET_VALUE: u8 = 0;
}
