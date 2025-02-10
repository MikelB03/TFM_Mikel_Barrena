#[doc = "Register `TNFEN` reader"]
pub type R = crate::R<TnfenSpec>;
#[doc = "Register `TNFEN` writer"]
pub type W = crate::W<TnfenSpec>;
#[doc = "Enabling or disabling use of the noise filter for the TI00 pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnfen00 {
    #[doc = "0: Turns the noise filter off"]
    _0 = 0,
    #[doc = "1: Turns the noise filter on"]
    _1 = 1,
}
impl From<Tnfen00> for bool {
    #[inline(always)]
    fn from(variant: Tnfen00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNFEN00` reader - Enabling or disabling use of the noise filter for the TI00 pin"]
pub type Tnfen00R = crate::BitReader<Tnfen00>;
impl Tnfen00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnfen00 {
        match self.bits {
            false => Tnfen00::_0,
            true => Tnfen00::_1,
        }
    }
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tnfen00::_0
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tnfen00::_1
    }
}
#[doc = "Field `TNFEN00` writer - Enabling or disabling use of the noise filter for the TI00 pin"]
pub type Tnfen00W<'a, REG> = crate::BitWriter<'a, REG, Tnfen00>;
impl<'a, REG> Tnfen00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen00::_0)
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen00::_1)
    }
}
#[doc = "Enabling or disabling use of the noise filter for the TI01 pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnfen01 {
    #[doc = "0: Turns the noise filter off"]
    _0 = 0,
    #[doc = "1: Turns the noise filter on"]
    _1 = 1,
}
impl From<Tnfen01> for bool {
    #[inline(always)]
    fn from(variant: Tnfen01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNFEN01` reader - Enabling or disabling use of the noise filter for the TI01 pin"]
pub type Tnfen01R = crate::BitReader<Tnfen01>;
impl Tnfen01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnfen01 {
        match self.bits {
            false => Tnfen01::_0,
            true => Tnfen01::_1,
        }
    }
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tnfen01::_0
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tnfen01::_1
    }
}
#[doc = "Field `TNFEN01` writer - Enabling or disabling use of the noise filter for the TI01 pin"]
pub type Tnfen01W<'a, REG> = crate::BitWriter<'a, REG, Tnfen01>;
impl<'a, REG> Tnfen01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen01::_0)
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen01::_1)
    }
}
#[doc = "Enabling or disabling use of the noise filter for the TI02 pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnfen02 {
    #[doc = "0: Turns the noise filter off"]
    _0 = 0,
    #[doc = "1: Turns the noise filter on"]
    _1 = 1,
}
impl From<Tnfen02> for bool {
    #[inline(always)]
    fn from(variant: Tnfen02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNFEN02` reader - Enabling or disabling use of the noise filter for the TI02 pin"]
pub type Tnfen02R = crate::BitReader<Tnfen02>;
impl Tnfen02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnfen02 {
        match self.bits {
            false => Tnfen02::_0,
            true => Tnfen02::_1,
        }
    }
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tnfen02::_0
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tnfen02::_1
    }
}
#[doc = "Field `TNFEN02` writer - Enabling or disabling use of the noise filter for the TI02 pin"]
pub type Tnfen02W<'a, REG> = crate::BitWriter<'a, REG, Tnfen02>;
impl<'a, REG> Tnfen02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen02::_0)
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen02::_1)
    }
}
#[doc = "Enabling or disabling use of the noise filter for the TI03 pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnfen03 {
    #[doc = "0: Turns the noise filter off"]
    _0 = 0,
    #[doc = "1: Turns the noise filter on"]
    _1 = 1,
}
impl From<Tnfen03> for bool {
    #[inline(always)]
    fn from(variant: Tnfen03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNFEN03` reader - Enabling or disabling use of the noise filter for the TI03 pin"]
pub type Tnfen03R = crate::BitReader<Tnfen03>;
impl Tnfen03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnfen03 {
        match self.bits {
            false => Tnfen03::_0,
            true => Tnfen03::_1,
        }
    }
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tnfen03::_0
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tnfen03::_1
    }
}
#[doc = "Field `TNFEN03` writer - Enabling or disabling use of the noise filter for the TI03 pin"]
pub type Tnfen03W<'a, REG> = crate::BitWriter<'a, REG, Tnfen03>;
impl<'a, REG> Tnfen03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen03::_0)
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen03::_1)
    }
}
#[doc = "Enabling or disabling use of the noise filter for the TI04 pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnfen04 {
    #[doc = "0: Turns the noise filter off"]
    _0 = 0,
    #[doc = "1: Turns the noise filter on"]
    _1 = 1,
}
impl From<Tnfen04> for bool {
    #[inline(always)]
    fn from(variant: Tnfen04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNFEN04` reader - Enabling or disabling use of the noise filter for the TI04 pin"]
pub type Tnfen04R = crate::BitReader<Tnfen04>;
impl Tnfen04R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnfen04 {
        match self.bits {
            false => Tnfen04::_0,
            true => Tnfen04::_1,
        }
    }
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tnfen04::_0
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tnfen04::_1
    }
}
#[doc = "Field `TNFEN04` writer - Enabling or disabling use of the noise filter for the TI04 pin"]
pub type Tnfen04W<'a, REG> = crate::BitWriter<'a, REG, Tnfen04>;
impl<'a, REG> Tnfen04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen04::_0)
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen04::_1)
    }
}
#[doc = "Enabling or disabling use of the noise filter for the TI05 pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnfen05 {
    #[doc = "0: Turns the noise filter off"]
    _0 = 0,
    #[doc = "1: Turns the noise filter on"]
    _1 = 1,
}
impl From<Tnfen05> for bool {
    #[inline(always)]
    fn from(variant: Tnfen05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNFEN05` reader - Enabling or disabling use of the noise filter for the TI05 pin"]
pub type Tnfen05R = crate::BitReader<Tnfen05>;
impl Tnfen05R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnfen05 {
        match self.bits {
            false => Tnfen05::_0,
            true => Tnfen05::_1,
        }
    }
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tnfen05::_0
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tnfen05::_1
    }
}
#[doc = "Field `TNFEN05` writer - Enabling or disabling use of the noise filter for the TI05 pin"]
pub type Tnfen05W<'a, REG> = crate::BitWriter<'a, REG, Tnfen05>;
impl<'a, REG> Tnfen05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen05::_0)
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen05::_1)
    }
}
#[doc = "Enabling or disabling use of the noise filter for the TI06 pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnfen06 {
    #[doc = "0: Turns the noise filter off"]
    _0 = 0,
    #[doc = "1: Turns the noise filter on"]
    _1 = 1,
}
impl From<Tnfen06> for bool {
    #[inline(always)]
    fn from(variant: Tnfen06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNFEN06` reader - Enabling or disabling use of the noise filter for the TI06 pin"]
pub type Tnfen06R = crate::BitReader<Tnfen06>;
impl Tnfen06R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnfen06 {
        match self.bits {
            false => Tnfen06::_0,
            true => Tnfen06::_1,
        }
    }
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tnfen06::_0
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tnfen06::_1
    }
}
#[doc = "Field `TNFEN06` writer - Enabling or disabling use of the noise filter for the TI06 pin"]
pub type Tnfen06W<'a, REG> = crate::BitWriter<'a, REG, Tnfen06>;
impl<'a, REG> Tnfen06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen06::_0)
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen06::_1)
    }
}
#[doc = "Enabling or disabling use of the noise filter for the TI07 pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnfen07 {
    #[doc = "0: Turns the noise filter off"]
    _0 = 0,
    #[doc = "1: Turns the noise filter on"]
    _1 = 1,
}
impl From<Tnfen07> for bool {
    #[inline(always)]
    fn from(variant: Tnfen07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNFEN07` reader - Enabling or disabling use of the noise filter for the TI07 pin"]
pub type Tnfen07R = crate::BitReader<Tnfen07>;
impl Tnfen07R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnfen07 {
        match self.bits {
            false => Tnfen07::_0,
            true => Tnfen07::_1,
        }
    }
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tnfen07::_0
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tnfen07::_1
    }
}
#[doc = "Field `TNFEN07` writer - Enabling or disabling use of the noise filter for the TI07 pin"]
pub type Tnfen07W<'a, REG> = crate::BitWriter<'a, REG, Tnfen07>;
impl<'a, REG> Tnfen07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turns the noise filter off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen07::_0)
    }
    #[doc = "Turns the noise filter on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tnfen07::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enabling or disabling use of the noise filter for the TI00 pin"]
    #[inline(always)]
    pub fn tnfen00(&self) -> Tnfen00R {
        Tnfen00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enabling or disabling use of the noise filter for the TI01 pin"]
    #[inline(always)]
    pub fn tnfen01(&self) -> Tnfen01R {
        Tnfen01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enabling or disabling use of the noise filter for the TI02 pin"]
    #[inline(always)]
    pub fn tnfen02(&self) -> Tnfen02R {
        Tnfen02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enabling or disabling use of the noise filter for the TI03 pin"]
    #[inline(always)]
    pub fn tnfen03(&self) -> Tnfen03R {
        Tnfen03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enabling or disabling use of the noise filter for the TI04 pin"]
    #[inline(always)]
    pub fn tnfen04(&self) -> Tnfen04R {
        Tnfen04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enabling or disabling use of the noise filter for the TI05 pin"]
    #[inline(always)]
    pub fn tnfen05(&self) -> Tnfen05R {
        Tnfen05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enabling or disabling use of the noise filter for the TI06 pin"]
    #[inline(always)]
    pub fn tnfen06(&self) -> Tnfen06R {
        Tnfen06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enabling or disabling use of the noise filter for the TI07 pin"]
    #[inline(always)]
    pub fn tnfen07(&self) -> Tnfen07R {
        Tnfen07R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enabling or disabling use of the noise filter for the TI00 pin"]
    #[inline(always)]
    pub fn tnfen00(&mut self) -> Tnfen00W<TnfenSpec> {
        Tnfen00W::new(self, 0)
    }
    #[doc = "Bit 1 - Enabling or disabling use of the noise filter for the TI01 pin"]
    #[inline(always)]
    pub fn tnfen01(&mut self) -> Tnfen01W<TnfenSpec> {
        Tnfen01W::new(self, 1)
    }
    #[doc = "Bit 2 - Enabling or disabling use of the noise filter for the TI02 pin"]
    #[inline(always)]
    pub fn tnfen02(&mut self) -> Tnfen02W<TnfenSpec> {
        Tnfen02W::new(self, 2)
    }
    #[doc = "Bit 3 - Enabling or disabling use of the noise filter for the TI03 pin"]
    #[inline(always)]
    pub fn tnfen03(&mut self) -> Tnfen03W<TnfenSpec> {
        Tnfen03W::new(self, 3)
    }
    #[doc = "Bit 4 - Enabling or disabling use of the noise filter for the TI04 pin"]
    #[inline(always)]
    pub fn tnfen04(&mut self) -> Tnfen04W<TnfenSpec> {
        Tnfen04W::new(self, 4)
    }
    #[doc = "Bit 5 - Enabling or disabling use of the noise filter for the TI05 pin"]
    #[inline(always)]
    pub fn tnfen05(&mut self) -> Tnfen05W<TnfenSpec> {
        Tnfen05W::new(self, 5)
    }
    #[doc = "Bit 6 - Enabling or disabling use of the noise filter for the TI06 pin"]
    #[inline(always)]
    pub fn tnfen06(&mut self) -> Tnfen06W<TnfenSpec> {
        Tnfen06W::new(self, 6)
    }
    #[doc = "Bit 7 - Enabling or disabling use of the noise filter for the TI07 pin"]
    #[inline(always)]
    pub fn tnfen07(&mut self) -> Tnfen07W<TnfenSpec> {
        Tnfen07W::new(self, 7)
    }
}
#[doc = "TAU Noise Filter Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tnfen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tnfen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnfenSpec;
impl crate::RegisterSpec for TnfenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tnfen::R`](R) reader structure"]
impl crate::Readable for TnfenSpec {}
#[doc = "`write(|w| ..)` method takes [`tnfen::W`](W) writer structure"]
impl crate::Writable for TnfenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TNFEN to value 0"]
impl crate::Resettable for TnfenSpec {
    const RESET_VALUE: u8 = 0;
}
