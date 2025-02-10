#[doc = "Register `ADS` reader"]
pub type R = crate::R<AdsSpec>;
#[doc = "Register `ADS` writer"]
pub type W = crate::W<AdsSpec>;
#[doc = "Field `ADS` reader - Selection of the analog input channel (See to )"]
pub type AdsR = crate::FieldReader;
#[doc = "Field `ADS` writer - Selection of the analog input channel (See to )"]
pub type AdsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Select internal or external of analog input (See to )\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adiss {
    #[doc = "0: External input"]
    _0 = 0,
    #[doc = "1: Internal circuit input"]
    _1 = 1,
}
impl From<Adiss> for bool {
    #[inline(always)]
    fn from(variant: Adiss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADISS` reader - Select internal or external of analog input (See to )"]
pub type AdissR = crate::BitReader<Adiss>;
impl AdissR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adiss {
        match self.bits {
            false => Adiss::_0,
            true => Adiss::_1,
        }
    }
    #[doc = "External input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adiss::_0
    }
    #[doc = "Internal circuit input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adiss::_1
    }
}
#[doc = "Field `ADISS` writer - Select internal or external of analog input (See to )"]
pub type AdissW<'a, REG> = crate::BitWriter<'a, REG, Adiss>;
impl<'a, REG> AdissW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adiss::_0)
    }
    #[doc = "Internal circuit input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adiss::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selection of the analog input channel (See to )"]
    #[inline(always)]
    pub fn ads(&self) -> AdsR {
        AdsR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 7 - Select internal or external of analog input (See to )"]
    #[inline(always)]
    pub fn adiss(&self) -> AdissR {
        AdissR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selection of the analog input channel (See to )"]
    #[inline(always)]
    pub fn ads(&mut self) -> AdsW<AdsSpec> {
        AdsW::new(self, 0)
    }
    #[doc = "Bit 7 - Select internal or external of analog input (See to )"]
    #[inline(always)]
    pub fn adiss(&mut self) -> AdissW<AdsSpec> {
        AdissW::new(self, 7)
    }
}
#[doc = "Analog Input Channel Specification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ads::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ads::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdsSpec;
impl crate::RegisterSpec for AdsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ads::R`](R) reader structure"]
impl crate::Readable for AdsSpec {}
#[doc = "`write(|w| ..)` method takes [`ads::W`](W) writer structure"]
impl crate::Writable for AdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADS to value 0"]
impl crate::Resettable for AdsSpec {
    const RESET_VALUE: u8 = 0;
}
