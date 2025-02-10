#[doc = "Register `ITLS0` reader"]
pub type R = crate::R<Itls0Spec>;
#[doc = "Register `ITLS0` writer"]
pub type W = crate::W<Itls0Spec>;
#[doc = "Compare match detection flag for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itf00 {
    #[doc = "0: A compare match signal has not been detected in channel 0."]
    _0 = 0,
    #[doc = "1: A compare match signal has been detected in channel 0."]
    _1 = 1,
}
impl From<Itf00> for bool {
    #[inline(always)]
    fn from(variant: Itf00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITF00` reader - Compare match detection flag for channel 0"]
pub type Itf00R = crate::BitReader<Itf00>;
impl Itf00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itf00 {
        match self.bits {
            false => Itf00::_0,
            true => Itf00::_1,
        }
    }
    #[doc = "A compare match signal has not been detected in channel 0."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Itf00::_0
    }
    #[doc = "A compare match signal has been detected in channel 0."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Itf00::_1
    }
}
#[doc = "Field `ITF00` writer - Compare match detection flag for channel 0"]
pub type Itf00W<'a, REG> = crate::BitWriter<'a, REG, Itf00>;
impl<'a, REG> Itf00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A compare match signal has not been detected in channel 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Itf00::_0)
    }
    #[doc = "A compare match signal has been detected in channel 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Itf00::_1)
    }
}
#[doc = "Compare match detection flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itf01 {
    #[doc = "0: A compare match signal has not been detected in channel 1."]
    _0 = 0,
    #[doc = "1: A compare match signal has been detected in channel 1."]
    _1 = 1,
}
impl From<Itf01> for bool {
    #[inline(always)]
    fn from(variant: Itf01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITF01` reader - Compare match detection flag for channel 1"]
pub type Itf01R = crate::BitReader<Itf01>;
impl Itf01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itf01 {
        match self.bits {
            false => Itf01::_0,
            true => Itf01::_1,
        }
    }
    #[doc = "A compare match signal has not been detected in channel 1."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Itf01::_0
    }
    #[doc = "A compare match signal has been detected in channel 1."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Itf01::_1
    }
}
#[doc = "Field `ITF01` writer - Compare match detection flag for channel 1"]
pub type Itf01W<'a, REG> = crate::BitWriter<'a, REG, Itf01>;
impl<'a, REG> Itf01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A compare match signal has not been detected in channel 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Itf01::_0)
    }
    #[doc = "A compare match signal has been detected in channel 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Itf01::_1)
    }
}
#[doc = "Compare match detection flag for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itf02 {
    #[doc = "0: A compare match signal has not been detected in channel 2."]
    _0 = 0,
    #[doc = "1: A compare match signal has been detected in channel 2."]
    _1 = 1,
}
impl From<Itf02> for bool {
    #[inline(always)]
    fn from(variant: Itf02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITF02` reader - Compare match detection flag for channel 2"]
pub type Itf02R = crate::BitReader<Itf02>;
impl Itf02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itf02 {
        match self.bits {
            false => Itf02::_0,
            true => Itf02::_1,
        }
    }
    #[doc = "A compare match signal has not been detected in channel 2."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Itf02::_0
    }
    #[doc = "A compare match signal has been detected in channel 2."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Itf02::_1
    }
}
#[doc = "Field `ITF02` writer - Compare match detection flag for channel 2"]
pub type Itf02W<'a, REG> = crate::BitWriter<'a, REG, Itf02>;
impl<'a, REG> Itf02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A compare match signal has not been detected in channel 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Itf02::_0)
    }
    #[doc = "A compare match signal has been detected in channel 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Itf02::_1)
    }
}
#[doc = "Compare match detection flag for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itf03 {
    #[doc = "0: A compare match signal has not been detected in channel 3."]
    _0 = 0,
    #[doc = "1: A compare match signal has been detected in channel 3."]
    _1 = 1,
}
impl From<Itf03> for bool {
    #[inline(always)]
    fn from(variant: Itf03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITF03` reader - Compare match detection flag for channel 3"]
pub type Itf03R = crate::BitReader<Itf03>;
impl Itf03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itf03 {
        match self.bits {
            false => Itf03::_0,
            true => Itf03::_1,
        }
    }
    #[doc = "A compare match signal has not been detected in channel 3."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Itf03::_0
    }
    #[doc = "A compare match signal has been detected in channel 3."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Itf03::_1
    }
}
#[doc = "Field `ITF03` writer - Compare match detection flag for channel 3"]
pub type Itf03W<'a, REG> = crate::BitWriter<'a, REG, Itf03>;
impl<'a, REG> Itf03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A compare match signal has not been detected in channel 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Itf03::_0)
    }
    #[doc = "A compare match signal has been detected in channel 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Itf03::_1)
    }
}
#[doc = "Capture detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itf0c {
    #[doc = "0: Completion of capturing has not been detected."]
    _0 = 0,
    #[doc = "1: Completion of capturing has been detected."]
    _1 = 1,
}
impl From<Itf0c> for bool {
    #[inline(always)]
    fn from(variant: Itf0c) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITF0C` reader - Capture detection flag"]
pub type Itf0cR = crate::BitReader<Itf0c>;
impl Itf0cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itf0c {
        match self.bits {
            false => Itf0c::_0,
            true => Itf0c::_1,
        }
    }
    #[doc = "Completion of capturing has not been detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Itf0c::_0
    }
    #[doc = "Completion of capturing has been detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Itf0c::_1
    }
}
#[doc = "Field `ITF0C` writer - Capture detection flag"]
pub type Itf0cW<'a, REG> = crate::BitWriter<'a, REG, Itf0c>;
impl<'a, REG> Itf0cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of capturing has not been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Itf0c::_0)
    }
    #[doc = "Completion of capturing has been detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Itf0c::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare match detection flag for channel 0"]
    #[inline(always)]
    pub fn itf00(&self) -> Itf00R {
        Itf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare match detection flag for channel 1"]
    #[inline(always)]
    pub fn itf01(&self) -> Itf01R {
        Itf01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare match detection flag for channel 2"]
    #[inline(always)]
    pub fn itf02(&self) -> Itf02R {
        Itf02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare match detection flag for channel 3"]
    #[inline(always)]
    pub fn itf03(&self) -> Itf03R {
        Itf03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture detection flag"]
    #[inline(always)]
    pub fn itf0c(&self) -> Itf0cR {
        Itf0cR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare match detection flag for channel 0"]
    #[inline(always)]
    pub fn itf00(&mut self) -> Itf00W<Itls0Spec> {
        Itf00W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare match detection flag for channel 1"]
    #[inline(always)]
    pub fn itf01(&mut self) -> Itf01W<Itls0Spec> {
        Itf01W::new(self, 1)
    }
    #[doc = "Bit 2 - Compare match detection flag for channel 2"]
    #[inline(always)]
    pub fn itf02(&mut self) -> Itf02W<Itls0Spec> {
        Itf02W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare match detection flag for channel 3"]
    #[inline(always)]
    pub fn itf03(&mut self) -> Itf03W<Itls0Spec> {
        Itf03W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture detection flag"]
    #[inline(always)]
    pub fn itf0c(&mut self) -> Itf0cW<Itls0Spec> {
        Itf0cW::new(self, 4)
    }
}
#[doc = "Interval Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`itls0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itls0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itls0Spec;
impl crate::RegisterSpec for Itls0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itls0::R`](R) reader structure"]
impl crate::Readable for Itls0Spec {}
#[doc = "`write(|w| ..)` method takes [`itls0::W`](W) writer structure"]
impl crate::Writable for Itls0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ITLS0 to value 0"]
impl crate::Resettable for Itls0Spec {
    const RESET_VALUE: u8 = 0;
}
