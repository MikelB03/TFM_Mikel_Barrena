#[doc = "Register `BUSCNTOAD` reader"]
pub type R = crate::R<BuscntoadSpec>;
#[doc = "Register `BUSCNTOAD` writer"]
pub type W = crate::W<BuscntoadSpec>;
#[doc = "Operation After Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oad {
    #[doc = "0: Non-maskable interrupt"]
    _0 = 0,
    #[doc = "1: Reset"]
    _1 = 1,
}
impl From<Oad> for bool {
    #[inline(always)]
    fn from(variant: Oad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAD` reader - Operation After Detection"]
pub type OadR = crate::BitReader<Oad>;
impl OadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oad {
        match self.bits {
            false => Oad::_0,
            true => Oad::_1,
        }
    }
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oad::_0
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oad::_1
    }
}
#[doc = "Field `OAD` writer - Operation After Detection"]
pub type OadW<'a, REG> = crate::BitWriter<'a, REG, Oad>;
impl<'a, REG> OadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oad::_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oad::_1)
    }
}
#[doc = "Field `KEY` reader - Key Code"]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - Key Code"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Operation After Detection"]
    #[inline(always)]
    pub fn oad(&self) -> OadR {
        OadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Operation After Detection"]
    #[inline(always)]
    pub fn oad(&mut self) -> OadW<BuscntoadSpec> {
        OadW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<BuscntoadSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Bus Control Error Operation After Detection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buscntoad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buscntoad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuscntoadSpec;
impl crate::RegisterSpec for BuscntoadSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`buscntoad::R`](R) reader structure"]
impl crate::Readable for BuscntoadSpec {}
#[doc = "`write(|w| ..)` method takes [`buscntoad::W`](W) writer structure"]
impl crate::Writable for BuscntoadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BUSCNTOAD to value 0"]
impl crate::Resettable for BuscntoadSpec {
    const RESET_VALUE: u16 = 0;
}
