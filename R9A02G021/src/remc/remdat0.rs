#[doc = "Register `REMDAT0` reader"]
pub type R = crate::R<Remdat0Spec>;
#[doc = "Register `REMDAT0` writer"]
pub type W = crate::W<Remdat0Spec>;
#[doc = "Receive Data 0 Store (bit 0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dat0 {
    #[doc = "0: This register is cleared (0x00)"]
    _0 = 0,
    #[doc = "1: Setting prohibited"]
    Others = 1,
}
impl From<Dat0> for bool {
    #[inline(always)]
    fn from(variant: Dat0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAT0` reader - Receive Data 0 Store (bit 0)"]
pub type Dat0R = crate::BitReader<Dat0>;
impl Dat0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dat0 {
        match self.bits {
            false => Dat0::_0,
            true => Dat0::Others,
        }
    }
    #[doc = "This register is cleared (0x00)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dat0::_0
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Dat0::Others
    }
}
#[doc = "Field `DAT0` writer - Receive Data 0 Store (bit 0)"]
pub type Dat0W<'a, REG> = crate::BitWriter<'a, REG, Dat0>;
impl<'a, REG> Dat0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This register is cleared (0x00)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dat0::_0)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Dat0::Others)
    }
}
#[doc = "Field `DAT` reader - Receive Data 0 Store (bit 7 to 1)"]
pub type DatR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive Data 0 Store (bit 0)"]
    #[inline(always)]
    pub fn dat0(&self) -> Dat0R {
        Dat0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Receive Data 0 Store (bit 7 to 1)"]
    #[inline(always)]
    pub fn dat(&self) -> DatR {
        DatR::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data 0 Store (bit 0)"]
    #[inline(always)]
    pub fn dat0(&mut self) -> Dat0W<Remdat0Spec> {
        Dat0W::new(self, 0)
    }
}
#[doc = "Receive Data 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remdat0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remdat0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remdat0Spec;
impl crate::RegisterSpec for Remdat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`remdat0::R`](R) reader structure"]
impl crate::Readable for Remdat0Spec {}
#[doc = "`write(|w| ..)` method takes [`remdat0::W`](W) writer structure"]
impl crate::Writable for Remdat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets REMDAT0 to value 0"]
impl crate::Resettable for Remdat0Spec {
    const RESET_VALUE: u8 = 0;
}
