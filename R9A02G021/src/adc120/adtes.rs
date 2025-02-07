#[doc = "Register `ADTES` reader"]
pub type R = crate::R<AdtesSpec>;
#[doc = "Register `ADTES` writer"]
pub type W = crate::W<AdtesSpec>;
#[doc = "Selection of A/D conversion target for testing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adtes {
    #[doc = "0: ANIxx, temperature sensor output voltage or internal reference voltage (Set by analog input channel specification register (ADS))"]
    _00 = 0,
    #[doc = "2: The ‘-’ side reference voltage (selected by the ADREFM bit of the ADM2 register)"]
    _10 = 2,
    #[doc = "3: The ‘+’ side reference voltage (selected by the ADREFP\\[1:0\\]
bits of the ADM2 register)"]
    _11 = 3,
    #[doc = "1: Setting prohibited."]
    Others = 1,
}
impl From<Adtes> for u8 {
    #[inline(always)]
    fn from(variant: Adtes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adtes {
    type Ux = u8;
}
impl crate::IsEnum for Adtes {}
#[doc = "Field `ADTES` reader - Selection of A/D conversion target for testing"]
pub type AdtesR = crate::FieldReader<Adtes>;
impl AdtesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adtes {
        match self.bits {
            0 => Adtes::_00,
            2 => Adtes::_10,
            3 => Adtes::_11,
            1 => Adtes::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "ANIxx, temperature sensor output voltage or internal reference voltage (Set by analog input channel specification register (ADS))"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Adtes::_00
    }
    #[doc = "The ‘-’ side reference voltage (selected by the ADREFM bit of the ADM2 register)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Adtes::_10
    }
    #[doc = "The ‘+’ side reference voltage (selected by the ADREFP\\[1:0\\]
bits of the ADM2 register)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Adtes::_11
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Adtes::Others
    }
}
#[doc = "Field `ADTES` writer - Selection of A/D conversion target for testing"]
pub type AdtesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adtes, crate::Safe>;
impl<'a, REG> AdtesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ANIxx, temperature sensor output voltage or internal reference voltage (Set by analog input channel specification register (ADS))"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Adtes::_00)
    }
    #[doc = "The ‘-’ side reference voltage (selected by the ADREFM bit of the ADM2 register)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Adtes::_10)
    }
    #[doc = "The ‘+’ side reference voltage (selected by the ADREFP\\[1:0\\]
bits of the ADM2 register)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Adtes::_11)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Adtes::Others)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selection of A/D conversion target for testing"]
    #[inline(always)]
    pub fn adtes(&self) -> AdtesR {
        AdtesR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selection of A/D conversion target for testing"]
    #[inline(always)]
    pub fn adtes(&mut self) -> AdtesW<AdtesSpec> {
        AdtesW::new(self, 0)
    }
}
#[doc = "A/D Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adtes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adtes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdtesSpec;
impl crate::RegisterSpec for AdtesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adtes::R`](R) reader structure"]
impl crate::Readable for AdtesSpec {}
#[doc = "`write(|w| ..)` method takes [`adtes::W`](W) writer structure"]
impl crate::Writable for AdtesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADTES to value 0"]
impl crate::Resettable for AdtesSpec {
    const RESET_VALUE: u8 = 0;
}
