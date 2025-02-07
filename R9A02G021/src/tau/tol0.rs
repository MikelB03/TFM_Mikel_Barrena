#[doc = "Register `TOL0` reader"]
pub type R = crate::R<Tol0Spec>;
#[doc = "Register `TOL0` writer"]
pub type W = crate::W<Tol0Spec>;
#[doc = "Control of timer output of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tol {
    #[doc = "0: Positive logic output (active-high)"]
    _0 = 0,
    #[doc = "1: Negative logic output (active-low)"]
    _1 = 1,
}
impl From<Tol> for u8 {
    #[inline(always)]
    fn from(variant: Tol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tol {
    type Ux = u8;
}
impl crate::IsEnum for Tol {}
#[doc = "Field `TOL` reader - Control of timer output of channel n"]
pub type TolR = crate::FieldReader<Tol>;
impl TolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tol> {
        match self.bits {
            0 => Some(Tol::_0),
            1 => Some(Tol::_1),
            _ => None,
        }
    }
    #[doc = "Positive logic output (active-high)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tol::_0
    }
    #[doc = "Negative logic output (active-low)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tol::_1
    }
}
#[doc = "Field `TOL` writer - Control of timer output of channel n"]
pub type TolW<'a, REG> = crate::FieldWriter<'a, REG, 7, Tol>;
impl<'a, REG> TolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Positive logic output (active-high)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tol::_0)
    }
    #[doc = "Negative logic output (active-low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tol::_1)
    }
}
impl R {
    #[doc = "Bits 1:7 - Control of timer output of channel n"]
    #[inline(always)]
    pub fn tol(&self) -> TolR {
        TolR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Control of timer output of channel n"]
    #[inline(always)]
    pub fn tol(&mut self) -> TolW<Tol0Spec> {
        TolW::new(self, 1)
    }
}
#[doc = "Timer Output Level Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tol0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tol0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tol0Spec;
impl crate::RegisterSpec for Tol0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tol0::R`](R) reader structure"]
impl crate::Readable for Tol0Spec {}
#[doc = "`write(|w| ..)` method takes [`tol0::W`](W) writer structure"]
impl crate::Writable for Tol0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TOL0 to value 0"]
impl crate::Resettable for Tol0Spec {
    const RESET_VALUE: u16 = 0;
}
