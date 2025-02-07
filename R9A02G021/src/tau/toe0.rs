#[doc = "Register `TOE0` reader"]
pub type R = crate::R<Toe0Spec>;
#[doc = "Register `TOE0` writer"]
pub type W = crate::W<Toe0Spec>;
#[doc = "Enabling or disabling timer output for channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Toe {
    #[doc = "0: Disables timer output"]
    _0 = 0,
    #[doc = "1: Enables timer output"]
    _1 = 1,
}
impl From<Toe> for u8 {
    #[inline(always)]
    fn from(variant: Toe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Toe {
    type Ux = u8;
}
impl crate::IsEnum for Toe {}
#[doc = "Field `TOE` reader - Enabling or disabling timer output for channel n"]
pub type ToeR = crate::FieldReader<Toe>;
impl ToeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Toe> {
        match self.bits {
            0 => Some(Toe::_0),
            1 => Some(Toe::_1),
            _ => None,
        }
    }
    #[doc = "Disables timer output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toe::_0
    }
    #[doc = "Enables timer output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toe::_1
    }
}
#[doc = "Field `TOE` writer - Enabling or disabling timer output for channel n"]
pub type ToeW<'a, REG> = crate::FieldWriter<'a, REG, 8, Toe>;
impl<'a, REG> ToeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disables timer output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toe::_0)
    }
    #[doc = "Enables timer output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toe::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Enabling or disabling timer output for channel n"]
    #[inline(always)]
    pub fn toe(&self) -> ToeR {
        ToeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enabling or disabling timer output for channel n"]
    #[inline(always)]
    pub fn toe(&mut self) -> ToeW<Toe0Spec> {
        ToeW::new(self, 0)
    }
}
#[doc = "Timer Output Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`toe0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`toe0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Toe0Spec;
impl crate::RegisterSpec for Toe0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`toe0::R`](R) reader structure"]
impl crate::Readable for Toe0Spec {}
#[doc = "`write(|w| ..)` method takes [`toe0::W`](W) writer structure"]
impl crate::Writable for Toe0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TOE0 to value 0"]
impl crate::Resettable for Toe0Spec {
    const RESET_VALUE: u16 = 0;
}
