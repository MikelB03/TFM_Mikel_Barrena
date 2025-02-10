#[doc = "Register `REMRBIT` reader"]
pub type R = crate::R<RemrbitSpec>;
#[doc = "Register `REMRBIT` writer"]
pub type W = crate::W<RemrbitSpec>;
#[doc = "Receive Bit Count Check (bit 0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rbit0 {
    #[doc = "0: This register is cleared (0x00)"]
    _0 = 0,
    #[doc = "1: Setting prohibited"]
    Others = 1,
}
impl From<Rbit0> for bool {
    #[inline(always)]
    fn from(variant: Rbit0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBIT0` reader - Receive Bit Count Check (bit 0)"]
pub type Rbit0R = crate::BitReader<Rbit0>;
impl Rbit0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbit0 {
        match self.bits {
            false => Rbit0::_0,
            true => Rbit0::Others,
        }
    }
    #[doc = "This register is cleared (0x00)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rbit0::_0
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Rbit0::Others
    }
}
#[doc = "Field `RBIT0` writer - Receive Bit Count Check (bit 0)"]
pub type Rbit0W<'a, REG> = crate::BitWriter<'a, REG, Rbit0>;
impl<'a, REG> Rbit0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This register is cleared (0x00)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rbit0::_0)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Rbit0::Others)
    }
}
#[doc = "Field `RBIT` reader - Receive Bit Count Check (bit 6 to 1)"]
pub type RbitR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive Bit Count Check (bit 0)"]
    #[inline(always)]
    pub fn rbit0(&self) -> Rbit0R {
        Rbit0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - Receive Bit Count Check (bit 6 to 1)"]
    #[inline(always)]
    pub fn rbit(&self) -> RbitR {
        RbitR::new((self.bits >> 1) & 0x3f)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Bit Count Check (bit 0)"]
    #[inline(always)]
    pub fn rbit0(&mut self) -> Rbit0W<RemrbitSpec> {
        Rbit0W::new(self, 0)
    }
}
#[doc = "Receive Bit Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remrbit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remrbit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemrbitSpec;
impl crate::RegisterSpec for RemrbitSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`remrbit::R`](R) reader structure"]
impl crate::Readable for RemrbitSpec {}
#[doc = "`write(|w| ..)` method takes [`remrbit::W`](W) writer structure"]
impl crate::Writable for RemrbitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets REMRBIT to value 0"]
impl crate::Resettable for RemrbitSpec {
    const RESET_VALUE: u8 = 0;
}
