#[doc = "Register `TRNGSCR1` reader"]
pub type R = crate::R<Trngscr1Spec>;
#[doc = "Register `TRNGSCR1` writer"]
pub type W = crate::W<Trngscr1Spec>;
#[doc = "TRNG Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Inten> for bool {
    #[inline(always)]
    fn from(variant: Inten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - TRNG Interrupt"]
pub type IntenR = crate::BitReader<Inten>;
impl IntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inten {
        match self.bits {
            false => Inten::_0,
            true => Inten::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Inten::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Inten::_1
    }
}
#[doc = "Field `INTEN` writer - TRNG Interrupt"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG, Inten>;
impl<'a, REG> IntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::_1)
    }
}
impl R {
    #[doc = "Bit 0 - TRNG Interrupt"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRNG Interrupt"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<Trngscr1Spec> {
        IntenW::new(self, 0)
    }
}
#[doc = "Random Number Seed Command Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`trngscr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trngscr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Trngscr1Spec;
impl crate::RegisterSpec for Trngscr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`trngscr1::R`](R) reader structure"]
impl crate::Readable for Trngscr1Spec {}
#[doc = "`write(|w| ..)` method takes [`trngscr1::W`](W) writer structure"]
impl crate::Writable for Trngscr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TRNGSCR1 to value 0x10"]
impl crate::Resettable for Trngscr1Spec {
    const RESET_VALUE: u8 = 0x10;
}
