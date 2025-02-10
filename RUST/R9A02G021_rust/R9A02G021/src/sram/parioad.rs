#[doc = "Register `PARIOAD` reader"]
pub type R = crate::R<ParioadSpec>;
#[doc = "Register `PARIOAD` writer"]
pub type W = crate::W<ParioadSpec>;
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
impl R {
    #[doc = "Bit 0 - Operation After Detection"]
    #[inline(always)]
    pub fn oad(&self) -> OadR {
        OadR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation After Detection"]
    #[inline(always)]
    pub fn oad(&mut self) -> OadW<ParioadSpec> {
        OadW::new(self, 0)
    }
}
#[doc = "SRAM Parity Error Operation After Detection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`parioad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parioad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParioadSpec;
impl crate::RegisterSpec for ParioadSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`parioad::R`](R) reader structure"]
impl crate::Readable for ParioadSpec {}
#[doc = "`write(|w| ..)` method takes [`parioad::W`](W) writer structure"]
impl crate::Writable for ParioadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PARIOAD to value 0"]
impl crate::Resettable for ParioadSpec {
    const RESET_VALUE: u8 = 0;
}
