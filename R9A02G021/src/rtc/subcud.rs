#[doc = "Register `SUBCUD` reader"]
pub type R = crate::R<SubcudSpec>;
#[doc = "Register `SUBCUD` writer"]
pub type W = crate::W<SubcudSpec>;
#[doc = "Field `F` reader - Adjustment Value"]
pub type FR = crate::FieldReader;
#[doc = "Field `F` writer - Adjustment Value"]
pub type FW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Setting of time error correction value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F6 {
    #[doc = "0: Increases by (F\\[5:0\\]
– 1) × 2"]
    _0 = 0,
    #[doc = "1: Decreases by (/F\\[5:0\\]
+ 1) × 2"]
    _1 = 1,
}
impl From<F6> for bool {
    #[inline(always)]
    fn from(variant: F6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F6` reader - Setting of time error correction value"]
pub type F6R = crate::BitReader<F6>;
impl F6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F6 {
        match self.bits {
            false => F6::_0,
            true => F6::_1,
        }
    }
    #[doc = "Increases by (F\\[5:0\\]
– 1) × 2"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == F6::_0
    }
    #[doc = "Decreases by (/F\\[5:0\\]
+ 1) × 2"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == F6::_1
    }
}
#[doc = "Field `F6` writer - Setting of time error correction value"]
pub type F6W<'a, REG> = crate::BitWriter<'a, REG, F6>;
impl<'a, REG> F6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Increases by (F\\[5:0\\]
– 1) × 2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(F6::_0)
    }
    #[doc = "Decreases by (/F\\[5:0\\]
+ 1) × 2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(F6::_1)
    }
}
#[doc = "Setting of time error correction timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dev {
    #[doc = "0: Corrects time error when the second digits are at 00, 20, or 40 (every 20 seconds)"]
    _0 = 0,
    #[doc = "1: Corrects time error only when the second digits are at 00 (every 60 seconds)"]
    _1 = 1,
}
impl From<Dev> for bool {
    #[inline(always)]
    fn from(variant: Dev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV` reader - Setting of time error correction timing"]
pub type DevR = crate::BitReader<Dev>;
impl DevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dev {
        match self.bits {
            false => Dev::_0,
            true => Dev::_1,
        }
    }
    #[doc = "Corrects time error when the second digits are at 00, 20, or 40 (every 20 seconds)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dev::_0
    }
    #[doc = "Corrects time error only when the second digits are at 00 (every 60 seconds)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dev::_1
    }
}
#[doc = "Field `DEV` writer - Setting of time error correction timing"]
pub type DevW<'a, REG> = crate::BitWriter<'a, REG, Dev>;
impl<'a, REG> DevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corrects time error when the second digits are at 00, 20, or 40 (every 20 seconds)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dev::_0)
    }
    #[doc = "Corrects time error only when the second digits are at 00 (every 60 seconds)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dev::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - Adjustment Value"]
    #[inline(always)]
    pub fn f(&self) -> FR {
        FR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Setting of time error correction value"]
    #[inline(always)]
    pub fn f6(&self) -> F6R {
        F6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Setting of time error correction timing"]
    #[inline(always)]
    pub fn dev(&self) -> DevR {
        DevR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Adjustment Value"]
    #[inline(always)]
    pub fn f(&mut self) -> FW<SubcudSpec> {
        FW::new(self, 0)
    }
    #[doc = "Bit 6 - Setting of time error correction value"]
    #[inline(always)]
    pub fn f6(&mut self) -> F6W<SubcudSpec> {
        F6W::new(self, 6)
    }
    #[doc = "Bit 7 - Setting of time error correction timing"]
    #[inline(always)]
    pub fn dev(&mut self) -> DevW<SubcudSpec> {
        DevW::new(self, 7)
    }
}
#[doc = "Time Error Correction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`subcud::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subcud::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubcudSpec;
impl crate::RegisterSpec for SubcudSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`subcud::R`](R) reader structure"]
impl crate::Readable for SubcudSpec {}
#[doc = "`write(|w| ..)` method takes [`subcud::W`](W) writer structure"]
impl crate::Writable for SubcudSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SUBCUD to value 0"]
impl crate::Resettable for SubcudSpec {
    const RESET_VALUE: u8 = 0;
}
