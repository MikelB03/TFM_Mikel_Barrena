#[doc = "Register `SNFEN` reader"]
pub type R = crate::R<SnfenSpec>;
#[doc = "Register `SNFEN` writer"]
pub type W = crate::W<SnfenSpec>;
#[doc = "Use of noise filter of RxD0 pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snfen00 {
    #[doc = "0: Noise filter OFF"]
    _0 = 0,
    #[doc = "1: Noise filter ON"]
    _1 = 1,
}
impl From<Snfen00> for bool {
    #[inline(always)]
    fn from(variant: Snfen00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNFEN00` reader - Use of noise filter of RxD0 pin"]
pub type Snfen00R = crate::BitReader<Snfen00>;
impl Snfen00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snfen00 {
        match self.bits {
            false => Snfen00::_0,
            true => Snfen00::_1,
        }
    }
    #[doc = "Noise filter OFF"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snfen00::_0
    }
    #[doc = "Noise filter ON"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snfen00::_1
    }
}
#[doc = "Field `SNFEN00` writer - Use of noise filter of RxD0 pin"]
pub type Snfen00W<'a, REG> = crate::BitWriter<'a, REG, Snfen00>;
impl<'a, REG> Snfen00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Noise filter OFF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snfen00::_0)
    }
    #[doc = "Noise filter ON"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snfen00::_1)
    }
}
#[doc = "Use of noise filter of RxD1 pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snfen10 {
    #[doc = "0: Noise filter OFF"]
    _0 = 0,
    #[doc = "1: Noise filter ON"]
    _1 = 1,
}
impl From<Snfen10> for bool {
    #[inline(always)]
    fn from(variant: Snfen10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNFEN10` reader - Use of noise filter of RxD1 pin"]
pub type Snfen10R = crate::BitReader<Snfen10>;
impl Snfen10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snfen10 {
        match self.bits {
            false => Snfen10::_0,
            true => Snfen10::_1,
        }
    }
    #[doc = "Noise filter OFF"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snfen10::_0
    }
    #[doc = "Noise filter ON"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snfen10::_1
    }
}
#[doc = "Field `SNFEN10` writer - Use of noise filter of RxD1 pin"]
pub type Snfen10W<'a, REG> = crate::BitWriter<'a, REG, Snfen10>;
impl<'a, REG> Snfen10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Noise filter OFF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snfen10::_0)
    }
    #[doc = "Noise filter ON"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snfen10::_1)
    }
}
#[doc = "Use of noise filter of RxD2 pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snfen20 {
    #[doc = "0: Noise filter OFF"]
    _0 = 0,
    #[doc = "1: Noise filter ON"]
    _1 = 1,
}
impl From<Snfen20> for bool {
    #[inline(always)]
    fn from(variant: Snfen20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNFEN20` reader - Use of noise filter of RxD2 pin"]
pub type Snfen20R = crate::BitReader<Snfen20>;
impl Snfen20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snfen20 {
        match self.bits {
            false => Snfen20::_0,
            true => Snfen20::_1,
        }
    }
    #[doc = "Noise filter OFF"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snfen20::_0
    }
    #[doc = "Noise filter ON"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snfen20::_1
    }
}
#[doc = "Field `SNFEN20` writer - Use of noise filter of RxD2 pin"]
pub type Snfen20W<'a, REG> = crate::BitWriter<'a, REG, Snfen20>;
impl<'a, REG> Snfen20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Noise filter OFF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snfen20::_0)
    }
    #[doc = "Noise filter ON"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snfen20::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Use of noise filter of RxD0 pin"]
    #[inline(always)]
    pub fn snfen00(&self) -> Snfen00R {
        Snfen00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Use of noise filter of RxD1 pin"]
    #[inline(always)]
    pub fn snfen10(&self) -> Snfen10R {
        Snfen10R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Use of noise filter of RxD2 pin"]
    #[inline(always)]
    pub fn snfen20(&self) -> Snfen20R {
        Snfen20R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use of noise filter of RxD0 pin"]
    #[inline(always)]
    pub fn snfen00(&mut self) -> Snfen00W<SnfenSpec> {
        Snfen00W::new(self, 0)
    }
    #[doc = "Bit 2 - Use of noise filter of RxD1 pin"]
    #[inline(always)]
    pub fn snfen10(&mut self) -> Snfen10W<SnfenSpec> {
        Snfen10W::new(self, 2)
    }
    #[doc = "Bit 4 - Use of noise filter of RxD2 pin"]
    #[inline(always)]
    pub fn snfen20(&mut self) -> Snfen20W<SnfenSpec> {
        Snfen20W::new(self, 4)
    }
}
#[doc = "SAU Noise Filter Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snfen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snfen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnfenSpec;
impl crate::RegisterSpec for SnfenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`snfen::R`](R) reader structure"]
impl crate::Readable for SnfenSpec {}
#[doc = "`write(|w| ..)` method takes [`snfen::W`](W) writer structure"]
impl crate::Writable for SnfenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SNFEN to value 0"]
impl crate::Resettable for SnfenSpec {
    const RESET_VALUE: u8 = 0;
}
