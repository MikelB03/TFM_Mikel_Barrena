#[doc = "Register `FISR` reader"]
pub type R = crate::R<FisrSpec>;
#[doc = "Register `FISR` writer"]
pub type W = crate::W<FisrSpec>;
#[doc = "Field `PCKA` reader - Flash-IF Clock Notification"]
pub type PckaR = crate::FieldReader;
#[doc = "Field `PCKA` writer - Flash-IF Clock Notification"]
pub type PckaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Startup Area Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sas {
    #[doc = "2: The startup area is switched to the default area temporarily"]
    _10 = 2,
    #[doc = "3: The startup area is switched to the alternate area temporarily."]
    _11 = 3,
    #[doc = "0: The startup area is selected according to the settings of the extra area."]
    Others = 0,
}
impl From<Sas> for u8 {
    #[inline(always)]
    fn from(variant: Sas) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sas {
    type Ux = u8;
}
impl crate::IsEnum for Sas {}
#[doc = "Field `SAS` reader - Startup Area Select"]
pub type SasR = crate::FieldReader<Sas>;
impl SasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sas {
        match self.bits {
            2 => Sas::_10,
            3 => Sas::_11,
            _ => Sas::Others,
        }
    }
    #[doc = "The startup area is switched to the default area temporarily"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sas::_10
    }
    #[doc = "The startup area is switched to the alternate area temporarily."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sas::_11
    }
    #[doc = "The startup area is selected according to the settings of the extra area."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Sas::Others)
    }
}
#[doc = "Field `SAS` writer - Startup Area Select"]
pub type SasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sas, crate::Safe>;
impl<'a, REG> SasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The startup area is switched to the default area temporarily"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sas::_10)
    }
    #[doc = "The startup area is switched to the alternate area temporarily."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sas::_11)
    }
    #[doc = "The startup area is selected according to the settings of the extra area."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Sas::Others)
    }
}
impl R {
    #[doc = "Bits 0:5 - Flash-IF Clock Notification"]
    #[inline(always)]
    pub fn pcka(&self) -> PckaR {
        PckaR::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - Startup Area Select"]
    #[inline(always)]
    pub fn sas(&self) -> SasR {
        SasR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - Flash-IF Clock Notification"]
    #[inline(always)]
    pub fn pcka(&mut self) -> PckaW<FisrSpec> {
        PckaW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Startup Area Select"]
    #[inline(always)]
    pub fn sas(&mut self) -> SasW<FisrSpec> {
        SasW::new(self, 6)
    }
}
#[doc = "Flash Initial Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FisrSpec;
impl crate::RegisterSpec for FisrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fisr::R`](R) reader structure"]
impl crate::Readable for FisrSpec {}
#[doc = "`write(|w| ..)` method takes [`fisr::W`](W) writer structure"]
impl crate::Writable for FisrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FISR to value 0"]
impl crate::Resettable for FisrSpec {
    const RESET_VALUE: u8 = 0;
}
