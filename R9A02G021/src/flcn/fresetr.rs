#[doc = "Register `FRESETR` reader"]
pub type R = crate::R<FresetrSpec>;
#[doc = "Register `FRESETR` writer"]
pub type W = crate::W<FresetrSpec>;
#[doc = "Software reset of the registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Freset {
    #[doc = "0: The registers related to the flash programming are not reset"]
    _0 = 0,
    #[doc = "1: The registers related to the flash programming are reset."]
    _1 = 1,
}
impl From<Freset> for bool {
    #[inline(always)]
    fn from(variant: Freset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRESET` reader - Software reset of the registers"]
pub type FresetR = crate::BitReader<Freset>;
impl FresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Freset {
        match self.bits {
            false => Freset::_0,
            true => Freset::_1,
        }
    }
    #[doc = "The registers related to the flash programming are not reset"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Freset::_0
    }
    #[doc = "The registers related to the flash programming are reset."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Freset::_1
    }
}
#[doc = "Field `FRESET` writer - Software reset of the registers"]
pub type FresetW<'a, REG> = crate::BitWriter<'a, REG, Freset>;
impl<'a, REG> FresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The registers related to the flash programming are not reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Freset::_0)
    }
    #[doc = "The registers related to the flash programming are reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Freset::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Software reset of the registers"]
    #[inline(always)]
    pub fn freset(&self) -> FresetR {
        FresetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset of the registers"]
    #[inline(always)]
    pub fn freset(&mut self) -> FresetW<FresetrSpec> {
        FresetW::new(self, 0)
    }
}
#[doc = "Flash Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fresetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fresetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FresetrSpec;
impl crate::RegisterSpec for FresetrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fresetr::R`](R) reader structure"]
impl crate::Readable for FresetrSpec {}
#[doc = "`write(|w| ..)` method takes [`fresetr::W`](W) writer structure"]
impl crate::Writable for FresetrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FRESETR to value 0"]
impl crate::Resettable for FresetrSpec {
    const RESET_VALUE: u8 = 0;
}
