#[doc = "Register `ITLMKF0` reader"]
pub type R = crate::R<Itlmkf0Spec>;
#[doc = "Register `ITLMKF0` writer"]
pub type W = crate::W<Itlmkf0Spec>;
#[doc = "Mask for compare match status flag for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mkf00 {
    #[doc = "0: ITLS0.ITF00 is not masked"]
    _0 = 0,
    #[doc = "1: ITLS0.ITF00 is masked"]
    _1 = 1,
}
impl From<Mkf00> for bool {
    #[inline(always)]
    fn from(variant: Mkf00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MKF00` reader - Mask for compare match status flag for channel 0"]
pub type Mkf00R = crate::BitReader<Mkf00>;
impl Mkf00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mkf00 {
        match self.bits {
            false => Mkf00::_0,
            true => Mkf00::_1,
        }
    }
    #[doc = "ITLS0.ITF00 is not masked"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mkf00::_0
    }
    #[doc = "ITLS0.ITF00 is masked"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mkf00::_1
    }
}
#[doc = "Field `MKF00` writer - Mask for compare match status flag for channel 0"]
pub type Mkf00W<'a, REG> = crate::BitWriter<'a, REG, Mkf00>;
impl<'a, REG> Mkf00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ITLS0.ITF00 is not masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mkf00::_0)
    }
    #[doc = "ITLS0.ITF00 is masked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mkf00::_1)
    }
}
#[doc = "Mask for compare match status flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mkf01 {
    #[doc = "0: ITLS0.ITF01 is not masked"]
    _0 = 0,
    #[doc = "1: ITLS0.ITF01 is masked"]
    _1 = 1,
}
impl From<Mkf01> for bool {
    #[inline(always)]
    fn from(variant: Mkf01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MKF01` reader - Mask for compare match status flag for channel 1"]
pub type Mkf01R = crate::BitReader<Mkf01>;
impl Mkf01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mkf01 {
        match self.bits {
            false => Mkf01::_0,
            true => Mkf01::_1,
        }
    }
    #[doc = "ITLS0.ITF01 is not masked"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mkf01::_0
    }
    #[doc = "ITLS0.ITF01 is masked"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mkf01::_1
    }
}
#[doc = "Field `MKF01` writer - Mask for compare match status flag for channel 1"]
pub type Mkf01W<'a, REG> = crate::BitWriter<'a, REG, Mkf01>;
impl<'a, REG> Mkf01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ITLS0.ITF01 is not masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mkf01::_0)
    }
    #[doc = "ITLS0.ITF01 is masked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mkf01::_1)
    }
}
#[doc = "Mask for compare match status flag for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mkf02 {
    #[doc = "0: ITLS0.ITF02 is not masked"]
    _0 = 0,
    #[doc = "1: ITLS0.ITF02 is masked"]
    _1 = 1,
}
impl From<Mkf02> for bool {
    #[inline(always)]
    fn from(variant: Mkf02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MKF02` reader - Mask for compare match status flag for channel 2"]
pub type Mkf02R = crate::BitReader<Mkf02>;
impl Mkf02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mkf02 {
        match self.bits {
            false => Mkf02::_0,
            true => Mkf02::_1,
        }
    }
    #[doc = "ITLS0.ITF02 is not masked"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mkf02::_0
    }
    #[doc = "ITLS0.ITF02 is masked"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mkf02::_1
    }
}
#[doc = "Field `MKF02` writer - Mask for compare match status flag for channel 2"]
pub type Mkf02W<'a, REG> = crate::BitWriter<'a, REG, Mkf02>;
impl<'a, REG> Mkf02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ITLS0.ITF02 is not masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mkf02::_0)
    }
    #[doc = "ITLS0.ITF02 is masked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mkf02::_1)
    }
}
#[doc = "Mask for compare match status flag for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mkf03 {
    #[doc = "0: ITLS0.ITF03 is not masked"]
    _0 = 0,
    #[doc = "1: ITLS0.ITF03 is masked"]
    _1 = 1,
}
impl From<Mkf03> for bool {
    #[inline(always)]
    fn from(variant: Mkf03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MKF03` reader - Mask for compare match status flag for channel 3"]
pub type Mkf03R = crate::BitReader<Mkf03>;
impl Mkf03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mkf03 {
        match self.bits {
            false => Mkf03::_0,
            true => Mkf03::_1,
        }
    }
    #[doc = "ITLS0.ITF03 is not masked"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mkf03::_0
    }
    #[doc = "ITLS0.ITF03 is masked"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mkf03::_1
    }
}
#[doc = "Field `MKF03` writer - Mask for compare match status flag for channel 3"]
pub type Mkf03W<'a, REG> = crate::BitWriter<'a, REG, Mkf03>;
impl<'a, REG> Mkf03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ITLS0.ITF03 is not masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mkf03::_0)
    }
    #[doc = "ITLS0.ITF03 is masked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mkf03::_1)
    }
}
#[doc = "Mask for capture detection status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mkf0c {
    #[doc = "0: ITLS0.ITF0C is not masked"]
    _0 = 0,
    #[doc = "1: ITLS0.ITF0C is masked"]
    _1 = 1,
}
impl From<Mkf0c> for bool {
    #[inline(always)]
    fn from(variant: Mkf0c) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MKF0C` reader - Mask for capture detection status flag"]
pub type Mkf0cR = crate::BitReader<Mkf0c>;
impl Mkf0cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mkf0c {
        match self.bits {
            false => Mkf0c::_0,
            true => Mkf0c::_1,
        }
    }
    #[doc = "ITLS0.ITF0C is not masked"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mkf0c::_0
    }
    #[doc = "ITLS0.ITF0C is masked"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mkf0c::_1
    }
}
#[doc = "Field `MKF0C` writer - Mask for capture detection status flag"]
pub type Mkf0cW<'a, REG> = crate::BitWriter<'a, REG, Mkf0c>;
impl<'a, REG> Mkf0cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ITLS0.ITF0C is not masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mkf0c::_0)
    }
    #[doc = "ITLS0.ITF0C is masked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mkf0c::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Mask for compare match status flag for channel 0"]
    #[inline(always)]
    pub fn mkf00(&self) -> Mkf00R {
        Mkf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask for compare match status flag for channel 1"]
    #[inline(always)]
    pub fn mkf01(&self) -> Mkf01R {
        Mkf01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask for compare match status flag for channel 2"]
    #[inline(always)]
    pub fn mkf02(&self) -> Mkf02R {
        Mkf02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask for compare match status flag for channel 3"]
    #[inline(always)]
    pub fn mkf03(&self) -> Mkf03R {
        Mkf03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask for capture detection status flag"]
    #[inline(always)]
    pub fn mkf0c(&self) -> Mkf0cR {
        Mkf0cR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for compare match status flag for channel 0"]
    #[inline(always)]
    pub fn mkf00(&mut self) -> Mkf00W<Itlmkf0Spec> {
        Mkf00W::new(self, 0)
    }
    #[doc = "Bit 1 - Mask for compare match status flag for channel 1"]
    #[inline(always)]
    pub fn mkf01(&mut self) -> Mkf01W<Itlmkf0Spec> {
        Mkf01W::new(self, 1)
    }
    #[doc = "Bit 2 - Mask for compare match status flag for channel 2"]
    #[inline(always)]
    pub fn mkf02(&mut self) -> Mkf02W<Itlmkf0Spec> {
        Mkf02W::new(self, 2)
    }
    #[doc = "Bit 3 - Mask for compare match status flag for channel 3"]
    #[inline(always)]
    pub fn mkf03(&mut self) -> Mkf03W<Itlmkf0Spec> {
        Mkf03W::new(self, 3)
    }
    #[doc = "Bit 4 - Mask for capture detection status flag"]
    #[inline(always)]
    pub fn mkf0c(&mut self) -> Mkf0cW<Itlmkf0Spec> {
        Mkf0cW::new(self, 4)
    }
}
#[doc = "Interval Timer Match Detection Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`itlmkf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlmkf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itlmkf0Spec;
impl crate::RegisterSpec for Itlmkf0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itlmkf0::R`](R) reader structure"]
impl crate::Readable for Itlmkf0Spec {}
#[doc = "`write(|w| ..)` method takes [`itlmkf0::W`](W) writer structure"]
impl crate::Writable for Itlmkf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ITLMKF0 to value 0"]
impl crate::Resettable for Itlmkf0Spec {
    const RESET_VALUE: u8 = 0;
}
