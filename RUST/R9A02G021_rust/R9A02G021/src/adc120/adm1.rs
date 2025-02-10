#[doc = "Register `ADM1` reader"]
pub type R = crate::R<Adm1Spec>;
#[doc = "Register `ADM1` writer"]
pub type W = crate::W<Adm1Spec>;
#[doc = "Selection of the hardware trigger signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adtrs {
    #[doc = "0: Timer Array Unit channel 1 count or capture end interrupt signal (TAU0_ENDI1)"]
    _000 = 0,
    #[doc = "2: Realtime clock interrupt signal (RTC_ALM)"]
    _010 = 2,
    #[doc = "3: 32-bit interval timer interrupt signal (TML32_OUTI)"]
    _011 = 3,
    #[doc = "4: Event input from ELC"]
    _100 = 4,
    #[doc = "1: Setting prohibited."]
    Others = 1,
}
impl From<Adtrs> for u8 {
    #[inline(always)]
    fn from(variant: Adtrs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adtrs {
    type Ux = u8;
}
impl crate::IsEnum for Adtrs {}
#[doc = "Field `ADTRS` reader - Selection of the hardware trigger signal"]
pub type AdtrsR = crate::FieldReader<Adtrs>;
impl AdtrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adtrs {
        match self.bits {
            0 => Adtrs::_000,
            2 => Adtrs::_010,
            3 => Adtrs::_011,
            4 => Adtrs::_100,
            _ => Adtrs::Others,
        }
    }
    #[doc = "Timer Array Unit channel 1 count or capture end interrupt signal (TAU0_ENDI1)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Adtrs::_000
    }
    #[doc = "Realtime clock interrupt signal (RTC_ALM)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Adtrs::_010
    }
    #[doc = "32-bit interval timer interrupt signal (TML32_OUTI)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Adtrs::_011
    }
    #[doc = "Event input from ELC"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Adtrs::_100
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Adtrs::Others)
    }
}
#[doc = "Field `ADTRS` writer - Selection of the hardware trigger signal"]
pub type AdtrsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adtrs, crate::Safe>;
impl<'a, REG> AdtrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer Array Unit channel 1 count or capture end interrupt signal (TAU0_ENDI1)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Adtrs::_000)
    }
    #[doc = "Realtime clock interrupt signal (RTC_ALM)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Adtrs::_010)
    }
    #[doc = "32-bit interval timer interrupt signal (TML32_OUTI)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Adtrs::_011)
    }
    #[doc = "Event input from ELC"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Adtrs::_100)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Adtrs::Others)
    }
}
#[doc = "PCLKB input frequency setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adlsp {
    #[doc = "0: 4 MHz < PCLKB ≤ 48 MHz"]
    _0 = 0,
    #[doc = "1: 1 MHz ≤ PCLKB ≤ 4 MHz"]
    _1 = 1,
}
impl From<Adlsp> for bool {
    #[inline(always)]
    fn from(variant: Adlsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADLSP` reader - PCLKB input frequency setting"]
pub type AdlspR = crate::BitReader<Adlsp>;
impl AdlspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adlsp {
        match self.bits {
            false => Adlsp::_0,
            true => Adlsp::_1,
        }
    }
    #[doc = "4 MHz < PCLKB ≤ 48 MHz"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adlsp::_0
    }
    #[doc = "1 MHz ≤ PCLKB ≤ 4 MHz"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adlsp::_1
    }
}
#[doc = "Field `ADLSP` writer - PCLKB input frequency setting"]
pub type AdlspW<'a, REG> = crate::BitWriter<'a, REG, Adlsp>;
impl<'a, REG> AdlspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4 MHz < PCLKB ≤ 48 MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsp::_0)
    }
    #[doc = "1 MHz ≤ PCLKB ≤ 4 MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsp::_1)
    }
}
#[doc = "Specification of the A/D conversion mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adscm {
    #[doc = "0: Sequential conversion mode"]
    _0 = 0,
    #[doc = "1: One-shot conversion mode"]
    _1 = 1,
}
impl From<Adscm> for bool {
    #[inline(always)]
    fn from(variant: Adscm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSCM` reader - Specification of the A/D conversion mode"]
pub type AdscmR = crate::BitReader<Adscm>;
impl AdscmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adscm {
        match self.bits {
            false => Adscm::_0,
            true => Adscm::_1,
        }
    }
    #[doc = "Sequential conversion mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adscm::_0
    }
    #[doc = "One-shot conversion mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adscm::_1
    }
}
#[doc = "Field `ADSCM` writer - Specification of the A/D conversion mode"]
pub type AdscmW<'a, REG> = crate::BitWriter<'a, REG, Adscm>;
impl<'a, REG> AdscmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sequential conversion mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adscm::_0)
    }
    #[doc = "One-shot conversion mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adscm::_1)
    }
}
#[doc = "Selection of the A/D conversion trigger mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adtmd {
    #[doc = "2: Hardware trigger no-wait mode"]
    _10 = 2,
    #[doc = "3: Hardware trigger wait mode"]
    _11 = 3,
    #[doc = "0: Software trigger no-wait mode or software trigger wait mode"]
    Others = 0,
}
impl From<Adtmd> for u8 {
    #[inline(always)]
    fn from(variant: Adtmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adtmd {
    type Ux = u8;
}
impl crate::IsEnum for Adtmd {}
#[doc = "Field `ADTMD` reader - Selection of the A/D conversion trigger mode"]
pub type AdtmdR = crate::FieldReader<Adtmd>;
impl AdtmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adtmd {
        match self.bits {
            2 => Adtmd::_10,
            3 => Adtmd::_11,
            _ => Adtmd::Others,
        }
    }
    #[doc = "Hardware trigger no-wait mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Adtmd::_10
    }
    #[doc = "Hardware trigger wait mode"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Adtmd::_11
    }
    #[doc = "Software trigger no-wait mode or software trigger wait mode"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Adtmd::Others)
    }
}
#[doc = "Field `ADTMD` writer - Selection of the A/D conversion trigger mode"]
pub type AdtmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adtmd, crate::Safe>;
impl<'a, REG> AdtmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Hardware trigger no-wait mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Adtmd::_10)
    }
    #[doc = "Hardware trigger wait mode"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Adtmd::_11)
    }
    #[doc = "Software trigger no-wait mode or software trigger wait mode"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Adtmd::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selection of the hardware trigger signal"]
    #[inline(always)]
    pub fn adtrs(&self) -> AdtrsR {
        AdtrsR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - PCLKB input frequency setting"]
    #[inline(always)]
    pub fn adlsp(&self) -> AdlspR {
        AdlspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Specification of the A/D conversion mode"]
    #[inline(always)]
    pub fn adscm(&self) -> AdscmR {
        AdscmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Selection of the A/D conversion trigger mode"]
    #[inline(always)]
    pub fn adtmd(&self) -> AdtmdR {
        AdtmdR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection of the hardware trigger signal"]
    #[inline(always)]
    pub fn adtrs(&mut self) -> AdtrsW<Adm1Spec> {
        AdtrsW::new(self, 0)
    }
    #[doc = "Bit 3 - PCLKB input frequency setting"]
    #[inline(always)]
    pub fn adlsp(&mut self) -> AdlspW<Adm1Spec> {
        AdlspW::new(self, 3)
    }
    #[doc = "Bit 5 - Specification of the A/D conversion mode"]
    #[inline(always)]
    pub fn adscm(&mut self) -> AdscmW<Adm1Spec> {
        AdscmW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Selection of the A/D conversion trigger mode"]
    #[inline(always)]
    pub fn adtmd(&mut self) -> AdtmdW<Adm1Spec> {
        AdtmdW::new(self, 6)
    }
}
#[doc = "A/D Converter Mode Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adm1Spec;
impl crate::RegisterSpec for Adm1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adm1::R`](R) reader structure"]
impl crate::Readable for Adm1Spec {}
#[doc = "`write(|w| ..)` method takes [`adm1::W`](W) writer structure"]
impl crate::Writable for Adm1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADM1 to value 0"]
impl crate::Resettable for Adm1Spec {
    const RESET_VALUE: u8 = 0;
}
