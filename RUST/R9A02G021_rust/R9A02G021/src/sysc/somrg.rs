#[doc = "Register `SOMRG` reader"]
pub type R = crate::R<SomrgSpec>;
#[doc = "Register `SOMRG` writer"]
pub type W = crate::W<SomrgSpec>;
#[doc = "Sub Clock Oscillator Margin check Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Soscmrg {
    #[doc = "0: Normal Current"]
    _00 = 0,
    #[doc = "1: Lower Margin check"]
    _01 = 1,
    #[doc = "2: Upper Margin check"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Soscmrg> for u8 {
    #[inline(always)]
    fn from(variant: Soscmrg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Soscmrg {
    type Ux = u8;
}
impl crate::IsEnum for Soscmrg {}
#[doc = "Field `SOSCMRG` reader - Sub Clock Oscillator Margin check Switching"]
pub type SoscmrgR = crate::FieldReader<Soscmrg>;
impl SoscmrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Soscmrg {
        match self.bits {
            0 => Soscmrg::_00,
            1 => Soscmrg::_01,
            2 => Soscmrg::_10,
            3 => Soscmrg::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal Current"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Soscmrg::_00
    }
    #[doc = "Lower Margin check"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Soscmrg::_01
    }
    #[doc = "Upper Margin check"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Soscmrg::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Soscmrg::_11
    }
}
#[doc = "Field `SOSCMRG` writer - Sub Clock Oscillator Margin check Switching"]
pub type SoscmrgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Soscmrg, crate::Safe>;
impl<'a, REG> SoscmrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Current"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Soscmrg::_00)
    }
    #[doc = "Lower Margin check"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Soscmrg::_01)
    }
    #[doc = "Upper Margin check"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Soscmrg::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Soscmrg::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sub Clock Oscillator Margin check Switching"]
    #[inline(always)]
    pub fn soscmrg(&self) -> SoscmrgR {
        SoscmrgR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sub Clock Oscillator Margin check Switching"]
    #[inline(always)]
    pub fn soscmrg(&mut self) -> SoscmrgW<SomrgSpec> {
        SoscmrgW::new(self, 0)
    }
}
#[doc = "Sub-Clock Oscillator Margin Check Register\n\nYou can [`read`](crate::Reg::read) this register and get [`somrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`somrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SomrgSpec;
impl crate::RegisterSpec for SomrgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`somrg::R`](R) reader structure"]
impl crate::Readable for SomrgSpec {}
#[doc = "`write(|w| ..)` method takes [`somrg::W`](W) writer structure"]
impl crate::Writable for SomrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SOMRG to value 0"]
impl crate::Resettable for SomrgSpec {
    const RESET_VALUE: u8 = 0;
}
