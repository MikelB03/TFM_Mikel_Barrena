#[doc = "Register `ADM2` reader"]
pub type R = crate::R<Adm2Spec>;
#[doc = "Register `ADM2` writer"]
pub type W = crate::W<Adm2Spec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adtyp {
    #[doc = "0: 10-bit resolution"]
    _00 = 0,
    #[doc = "1: 8-bit resolution"]
    _01 = 1,
    #[doc = "2: 12-bit resolution"]
    _10 = 2,
    #[doc = "3: Setting prohibited."]
    Others = 3,
}
impl From<Adtyp> for u8 {
    #[inline(always)]
    fn from(variant: Adtyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adtyp {
    type Ux = u8;
}
impl crate::IsEnum for Adtyp {}
#[doc = "Field `ADTYP` reader - "]
pub type AdtypR = crate::FieldReader<Adtyp>;
impl AdtypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adtyp {
        match self.bits {
            0 => Adtyp::_00,
            1 => Adtyp::_01,
            2 => Adtyp::_10,
            3 => Adtyp::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Adtyp::_00
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Adtyp::_01
    }
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Adtyp::_10
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Adtyp::Others
    }
}
#[doc = "Field `ADTYP` writer - "]
pub type AdtypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adtyp, crate::Safe>;
impl<'a, REG> AdtypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Adtyp::_00)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Adtyp::_01)
    }
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Adtyp::_10)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Adtyp::Others)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awc {
    #[doc = "0: Do not use the Snooze mode function."]
    _0 = 0,
    #[doc = "1: Use the Snooze mode function."]
    _1 = 1,
}
impl From<Awc> for bool {
    #[inline(always)]
    fn from(variant: Awc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWC` reader - "]
pub type AwcR = crate::BitReader<Awc>;
impl AwcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awc {
        match self.bits {
            false => Awc::_0,
            true => Awc::_1,
        }
    }
    #[doc = "Do not use the Snooze mode function."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Awc::_0
    }
    #[doc = "Use the Snooze mode function."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Awc::_1
    }
}
#[doc = "Field `AWC` writer - "]
pub type AwcW<'a, REG> = crate::BitWriter<'a, REG, Awc>;
impl<'a, REG> AwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not use the Snooze mode function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Awc::_0)
    }
    #[doc = "Use the Snooze mode function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Awc::_1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adrck {
    #[doc = "0: The interrupt signal (ADC_ENDI) is output when the ADLL register ≤ the ADCRn register ≤ the ADUL register (AREA 1)."]
    _0 = 0,
    #[doc = "1: The interrupt signal (ADC_ENDI) is output when the ADCRn register ≤ the ADLL register (AREA 2) or the ADUL register < the ADCRn register (AREA 3)."]
    _1 = 1,
}
impl From<Adrck> for bool {
    #[inline(always)]
    fn from(variant: Adrck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRCK` reader - "]
pub type AdrckR = crate::BitReader<Adrck>;
impl AdrckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adrck {
        match self.bits {
            false => Adrck::_0,
            true => Adrck::_1,
        }
    }
    #[doc = "The interrupt signal (ADC_ENDI) is output when the ADLL register ≤ the ADCRn register ≤ the ADUL register (AREA 1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adrck::_0
    }
    #[doc = "The interrupt signal (ADC_ENDI) is output when the ADCRn register ≤ the ADLL register (AREA 2) or the ADUL register < the ADCRn register (AREA 3)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adrck::_1
    }
}
#[doc = "Field `ADRCK` writer - "]
pub type AdrckW<'a, REG> = crate::BitWriter<'a, REG, Adrck>;
impl<'a, REG> AdrckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt signal (ADC_ENDI) is output when the ADLL register ≤ the ADCRn register ≤ the ADUL register (AREA 1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrck::_0)
    }
    #[doc = "The interrupt signal (ADC_ENDI) is output when the ADCRn register ≤ the ADLL register (AREA 2) or the ADUL register < the ADCRn register (AREA 3)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adrck::_1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adrefm {
    #[doc = "0: Supplied from VSS"]
    _0 = 0,
    #[doc = "1: Supplied from AVREFM"]
    _1 = 1,
}
impl From<Adrefm> for bool {
    #[inline(always)]
    fn from(variant: Adrefm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADREFM` reader - "]
pub type AdrefmR = crate::BitReader<Adrefm>;
impl AdrefmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adrefm {
        match self.bits {
            false => Adrefm::_0,
            true => Adrefm::_1,
        }
    }
    #[doc = "Supplied from VSS"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adrefm::_0
    }
    #[doc = "Supplied from AVREFM"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adrefm::_1
    }
}
#[doc = "Field `ADREFM` writer - "]
pub type AdrefmW<'a, REG> = crate::BitWriter<'a, REG, Adrefm>;
impl<'a, REG> AdrefmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Supplied from VSS"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrefm::_0)
    }
    #[doc = "Supplied from AVREFM"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adrefm::_1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adrefp {
    #[doc = "0: Supplied from VCC"]
    _00 = 0,
    #[doc = "1: Supplied from AVREFP"]
    _01 = 1,
    #[doc = "2: Supplied from the internal reference voltage"]
    _10 = 2,
    #[doc = "3: Discharge the internal circuitry"]
    _11 = 3,
}
impl From<Adrefp> for u8 {
    #[inline(always)]
    fn from(variant: Adrefp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adrefp {
    type Ux = u8;
}
impl crate::IsEnum for Adrefp {}
#[doc = "Field `ADREFP` reader - "]
pub type AdrefpR = crate::FieldReader<Adrefp>;
impl AdrefpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adrefp {
        match self.bits {
            0 => Adrefp::_00,
            1 => Adrefp::_01,
            2 => Adrefp::_10,
            3 => Adrefp::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Supplied from VCC"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Adrefp::_00
    }
    #[doc = "Supplied from AVREFP"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Adrefp::_01
    }
    #[doc = "Supplied from the internal reference voltage"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Adrefp::_10
    }
    #[doc = "Discharge the internal circuitry"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Adrefp::_11
    }
}
#[doc = "Field `ADREFP` writer - "]
pub type AdrefpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adrefp, crate::Safe>;
impl<'a, REG> AdrefpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Supplied from VCC"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Adrefp::_00)
    }
    #[doc = "Supplied from AVREFP"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Adrefp::_01)
    }
    #[doc = "Supplied from the internal reference voltage"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Adrefp::_10)
    }
    #[doc = "Discharge the internal circuitry"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Adrefp::_11)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adtyp(&self) -> AdtypR {
        AdtypR::new(self.bits & 3)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn awc(&self) -> AwcR {
        AwcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adrck(&self) -> AdrckR {
        AdrckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adrefm(&self) -> AdrefmR {
        AdrefmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn adrefp(&self) -> AdrefpR {
        AdrefpR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adtyp(&mut self) -> AdtypW<Adm2Spec> {
        AdtypW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn awc(&mut self) -> AwcW<Adm2Spec> {
        AwcW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adrck(&mut self) -> AdrckW<Adm2Spec> {
        AdrckW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adrefm(&mut self) -> AdrefmW<Adm2Spec> {
        AdrefmW::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn adrefp(&mut self) -> AdrefpW<Adm2Spec> {
        AdrefpW::new(self, 6)
    }
}
#[doc = "A/D Converter Mode Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adm2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adm2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adm2Spec;
impl crate::RegisterSpec for Adm2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adm2::R`](R) reader structure"]
impl crate::Readable for Adm2Spec {}
#[doc = "`write(|w| ..)` method takes [`adm2::W`](W) writer structure"]
impl crate::Writable for Adm2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADM2 to value 0"]
impl crate::Resettable for Adm2Spec {
    const RESET_VALUE: u8 = 0;
}
