#[doc = "Register `ITLCC0` reader"]
pub type R = crate::R<Itlcc0Spec>;
#[doc = "Register `ITLCC0` writer"]
pub type W = crate::W<Itlcc0Spec>;
#[doc = "Selection of capture trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctrs {
    #[doc = "0: Software trigger"]
    _00 = 0,
    #[doc = "1: Interrupt on compare match with ITLCMP01"]
    _01 = 1,
    #[doc = "2: TML32LCLK/TML32SCLK (rising edge)"]
    _10 = 2,
    #[doc = "3: Event input from ELC (rising edge)"]
    _11 = 3,
}
impl From<Ctrs> for u8 {
    #[inline(always)]
    fn from(variant: Ctrs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctrs {
    type Ux = u8;
}
impl crate::IsEnum for Ctrs {}
#[doc = "Field `CTRS` reader - Selection of capture trigger"]
pub type CtrsR = crate::FieldReader<Ctrs>;
impl CtrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrs {
        match self.bits {
            0 => Ctrs::_00,
            1 => Ctrs::_01,
            2 => Ctrs::_10,
            3 => Ctrs::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ctrs::_00
    }
    #[doc = "Interrupt on compare match with ITLCMP01"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ctrs::_01
    }
    #[doc = "TML32LCLK/TML32SCLK (rising edge)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ctrs::_10
    }
    #[doc = "Event input from ELC (rising edge)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ctrs::_11
    }
}
#[doc = "Field `CTRS` writer - Selection of capture trigger"]
pub type CtrsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctrs, crate::Safe>;
impl<'a, REG> CtrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrs::_00)
    }
    #[doc = "Interrupt on compare match with ITLCMP01"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrs::_01)
    }
    #[doc = "TML32LCLK/TML32SCLK (rising edge)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrs::_10)
    }
    #[doc = "Event input from ELC (rising edge)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrs::_11)
    }
}
#[doc = "Selection of capture counter clearing after capturing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capc0cr {
    #[doc = "0: The capture counter value is held after the completion of capturing"]
    _0 = 0,
    #[doc = "1: The capture counter value is cleared after the completion of capturing"]
    _1 = 1,
}
impl From<Capc0cr> for bool {
    #[inline(always)]
    fn from(variant: Capc0cr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPC0CR` reader - Selection of capture counter clearing after capturing"]
pub type Capc0crR = crate::BitReader<Capc0cr>;
impl Capc0crR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capc0cr {
        match self.bits {
            false => Capc0cr::_0,
            true => Capc0cr::_1,
        }
    }
    #[doc = "The capture counter value is held after the completion of capturing"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Capc0cr::_0
    }
    #[doc = "The capture counter value is cleared after the completion of capturing"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Capc0cr::_1
    }
}
#[doc = "Field `CAPC0CR` writer - Selection of capture counter clearing after capturing"]
pub type Capc0crW<'a, REG> = crate::BitWriter<'a, REG, Capc0cr>;
impl<'a, REG> Capc0crW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The capture counter value is held after the completion of capturing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Capc0cr::_0)
    }
    #[doc = "The capture counter value is cleared after the completion of capturing"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Capc0cr::_1)
    }
}
#[doc = "Software capture trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capr0 {
    #[doc = "0: Trigger operation does not proceed"]
    _0 = 0,
    #[doc = "1: A software trigger for capturing is generated"]
    _1 = 1,
}
impl From<Capr0> for bool {
    #[inline(always)]
    fn from(variant: Capr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPR0` reader - Software capture trigger"]
pub type Capr0R = crate::BitReader<Capr0>;
impl Capr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capr0 {
        match self.bits {
            false => Capr0::_0,
            true => Capr0::_1,
        }
    }
    #[doc = "Trigger operation does not proceed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Capr0::_0
    }
    #[doc = "A software trigger for capturing is generated"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Capr0::_1
    }
}
#[doc = "Field `CAPR0` writer - Software capture trigger"]
pub type Capr0W<'a, REG> = crate::BitWriter<'a, REG, Capr0>;
impl<'a, REG> Capr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger operation does not proceed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Capr0::_0)
    }
    #[doc = "A software trigger for capturing is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Capr0::_1)
    }
}
#[doc = "Capture completion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capf0 {
    #[doc = "0: Capturing has not been completed."]
    _0 = 0,
    #[doc = "1: Capturing has been completed. This flag is set to 1 after a capture trigger selected in the CTRS\\[1:0\\]
bits is generated and the captured data is stored in ITLCAP00. Writing 1 to the CAPF0CR bit clears this flag to 0."]
    _1 = 1,
}
impl From<Capf0> for bool {
    #[inline(always)]
    fn from(variant: Capf0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPF0` reader - Capture completion flag"]
pub type Capf0R = crate::BitReader<Capf0>;
impl Capf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capf0 {
        match self.bits {
            false => Capf0::_0,
            true => Capf0::_1,
        }
    }
    #[doc = "Capturing has not been completed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Capf0::_0
    }
    #[doc = "Capturing has been completed. This flag is set to 1 after a capture trigger selected in the CTRS\\[1:0\\]
bits is generated and the captured data is stored in ITLCAP00. Writing 1 to the CAPF0CR bit clears this flag to 0."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Capf0::_1
    }
}
#[doc = "Capture completion flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capf0cr {
    #[doc = "0: The value of the capture completion flag CAPF0 is held"]
    _0 = 0,
    #[doc = "1: The value of the capture completion flag CAPF0 is cleared"]
    _1 = 1,
}
impl From<Capf0cr> for bool {
    #[inline(always)]
    fn from(variant: Capf0cr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPF0CR` reader - Capture completion flag clear"]
pub type Capf0crR = crate::BitReader<Capf0cr>;
impl Capf0crR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capf0cr {
        match self.bits {
            false => Capf0cr::_0,
            true => Capf0cr::_1,
        }
    }
    #[doc = "The value of the capture completion flag CAPF0 is held"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Capf0cr::_0
    }
    #[doc = "The value of the capture completion flag CAPF0 is cleared"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Capf0cr::_1
    }
}
#[doc = "Field `CAPF0CR` writer - Capture completion flag clear"]
pub type Capf0crW<'a, REG> = crate::BitWriter<'a, REG, Capf0cr>;
impl<'a, REG> Capf0crW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The value of the capture completion flag CAPF0 is held"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Capf0cr::_0)
    }
    #[doc = "The value of the capture completion flag CAPF0 is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Capf0cr::_1)
    }
}
#[doc = "Capture enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capen0 {
    #[doc = "0: Capturing is disabled"]
    _0 = 0,
    #[doc = "1: Capturing is enabled"]
    _1 = 1,
}
impl From<Capen0> for bool {
    #[inline(always)]
    fn from(variant: Capen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPEN0` reader - Capture enable"]
pub type Capen0R = crate::BitReader<Capen0>;
impl Capen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capen0 {
        match self.bits {
            false => Capen0::_0,
            true => Capen0::_1,
        }
    }
    #[doc = "Capturing is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Capen0::_0
    }
    #[doc = "Capturing is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Capen0::_1
    }
}
#[doc = "Field `CAPEN0` writer - Capture enable"]
pub type Capen0W<'a, REG> = crate::BitWriter<'a, REG, Capen0>;
impl<'a, REG> Capen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capturing is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Capen0::_0)
    }
    #[doc = "Capturing is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Capen0::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selection of capture trigger"]
    #[inline(always)]
    pub fn ctrs(&self) -> CtrsR {
        CtrsR::new(self.bits & 3)
    }
    #[doc = "Bit 3 - Selection of capture counter clearing after capturing"]
    #[inline(always)]
    pub fn capc0cr(&self) -> Capc0crR {
        Capc0crR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software capture trigger"]
    #[inline(always)]
    pub fn capr0(&self) -> Capr0R {
        Capr0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture completion flag"]
    #[inline(always)]
    pub fn capf0(&self) -> Capf0R {
        Capf0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture completion flag clear"]
    #[inline(always)]
    pub fn capf0cr(&self) -> Capf0crR {
        Capf0crR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture enable"]
    #[inline(always)]
    pub fn capen0(&self) -> Capen0R {
        Capen0R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selection of capture trigger"]
    #[inline(always)]
    pub fn ctrs(&mut self) -> CtrsW<Itlcc0Spec> {
        CtrsW::new(self, 0)
    }
    #[doc = "Bit 3 - Selection of capture counter clearing after capturing"]
    #[inline(always)]
    pub fn capc0cr(&mut self) -> Capc0crW<Itlcc0Spec> {
        Capc0crW::new(self, 3)
    }
    #[doc = "Bit 4 - Software capture trigger"]
    #[inline(always)]
    pub fn capr0(&mut self) -> Capr0W<Itlcc0Spec> {
        Capr0W::new(self, 4)
    }
    #[doc = "Bit 6 - Capture completion flag clear"]
    #[inline(always)]
    pub fn capf0cr(&mut self) -> Capf0crW<Itlcc0Spec> {
        Capf0crW::new(self, 6)
    }
    #[doc = "Bit 7 - Capture enable"]
    #[inline(always)]
    pub fn capen0(&mut self) -> Capen0W<Itlcc0Spec> {
        Capen0W::new(self, 7)
    }
}
#[doc = "Interval Timer Capture Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`itlcc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlcc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itlcc0Spec;
impl crate::RegisterSpec for Itlcc0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itlcc0::R`](R) reader structure"]
impl crate::Readable for Itlcc0Spec {}
#[doc = "`write(|w| ..)` method takes [`itlcc0::W`](W) writer structure"]
impl crate::Writable for Itlcc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ITLCC0 to value 0"]
impl crate::Resettable for Itlcc0Spec {
    const RESET_VALUE: u8 = 0;
}
