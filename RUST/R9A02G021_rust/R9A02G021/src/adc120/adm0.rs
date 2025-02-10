#[doc = "Register `ADM0` reader"]
pub type R = crate::R<Adm0Spec>;
#[doc = "Register `ADM0` writer"]
pub type W = crate::W<Adm0Spec>;
#[doc = "A/D voltage comparator operation control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adce {
    #[doc = "0: Stops A/D voltage comparator operation"]
    _0 = 0,
    #[doc = "1: Enables A/D voltage comparator operation"]
    _1 = 1,
}
impl From<Adce> for bool {
    #[inline(always)]
    fn from(variant: Adce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCE` reader - A/D voltage comparator operation control"]
pub type AdceR = crate::BitReader<Adce>;
impl AdceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adce {
        match self.bits {
            false => Adce::_0,
            true => Adce::_1,
        }
    }
    #[doc = "Stops A/D voltage comparator operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adce::_0
    }
    #[doc = "Enables A/D voltage comparator operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adce::_1
    }
}
#[doc = "Field `ADCE` writer - A/D voltage comparator operation control"]
pub type AdceW<'a, REG> = crate::BitWriter<'a, REG, Adce>;
impl<'a, REG> AdceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stops A/D voltage comparator operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adce::_0)
    }
    #[doc = "Enables A/D voltage comparator operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adce::_1)
    }
}
#[doc = "Select Operation voltage mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lv {
    #[doc = "0: Normal mode 1"]
    _00 = 0,
    #[doc = "1: Normal mode 2"]
    _01 = 1,
    #[doc = "2: Low voltage mode 1"]
    _10 = 2,
    #[doc = "3: Low voltage mode 2"]
    _11 = 3,
}
impl From<Lv> for u8 {
    #[inline(always)]
    fn from(variant: Lv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lv {
    type Ux = u8;
}
impl crate::IsEnum for Lv {}
#[doc = "Field `LV` reader - Select Operation voltage mode"]
pub type LvR = crate::FieldReader<Lv>;
impl LvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lv {
        match self.bits {
            0 => Lv::_00,
            1 => Lv::_01,
            2 => Lv::_10,
            3 => Lv::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode 1"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Lv::_00
    }
    #[doc = "Normal mode 2"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Lv::_01
    }
    #[doc = "Low voltage mode 1"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Lv::_10
    }
    #[doc = "Low voltage mode 2"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Lv::_11
    }
}
#[doc = "Field `LV` writer - Select Operation voltage mode"]
pub type LvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lv, crate::Safe>;
impl<'a, REG> LvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode 1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Lv::_00)
    }
    #[doc = "Normal mode 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Lv::_01)
    }
    #[doc = "Low voltage mode 1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Lv::_10)
    }
    #[doc = "Low voltage mode 2"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Lv::_11)
    }
}
#[doc = "Select Conversion Clock (fAD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fr {
    #[doc = "0: PCLKB/32"]
    _000 = 0,
    #[doc = "1: PCLKB/16"]
    _001 = 1,
    #[doc = "2: PCLKB/8"]
    _010 = 2,
    #[doc = "3: PCLKB/4"]
    _011 = 3,
    #[doc = "4: PCLKB/2"]
    _100 = 4,
    #[doc = "5: PCLKB"]
    _101 = 5,
    #[doc = "6: Setting prohibited."]
    Others = 6,
}
impl From<Fr> for u8 {
    #[inline(always)]
    fn from(variant: Fr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fr {
    type Ux = u8;
}
impl crate::IsEnum for Fr {}
#[doc = "Field `FR` reader - Select Conversion Clock (fAD)"]
pub type FrR = crate::FieldReader<Fr>;
impl FrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fr {
        match self.bits {
            0 => Fr::_000,
            1 => Fr::_001,
            2 => Fr::_010,
            3 => Fr::_011,
            4 => Fr::_100,
            5 => Fr::_101,
            _ => Fr::Others,
        }
    }
    #[doc = "PCLKB/32"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Fr::_000
    }
    #[doc = "PCLKB/16"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Fr::_001
    }
    #[doc = "PCLKB/8"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Fr::_010
    }
    #[doc = "PCLKB/4"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Fr::_011
    }
    #[doc = "PCLKB/2"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Fr::_100
    }
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Fr::_101
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Fr::Others)
    }
}
#[doc = "Field `FR` writer - Select Conversion Clock (fAD)"]
pub type FrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fr, crate::Safe>;
impl<'a, REG> FrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLKB/32"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Fr::_000)
    }
    #[doc = "PCLKB/16"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Fr::_001)
    }
    #[doc = "PCLKB/8"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Fr::_010)
    }
    #[doc = "PCLKB/4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Fr::_011)
    }
    #[doc = "PCLKB/2"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Fr::_100)
    }
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Fr::_101)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Fr::Others)
    }
}
#[doc = "Specification of the A/D conversion channel selection mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admd {
    #[doc = "0: Select mode"]
    _0 = 0,
    #[doc = "1: Scan mode"]
    _1 = 1,
}
impl From<Admd> for bool {
    #[inline(always)]
    fn from(variant: Admd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMD` reader - Specification of the A/D conversion channel selection mode"]
pub type AdmdR = crate::BitReader<Admd>;
impl AdmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admd {
        match self.bits {
            false => Admd::_0,
            true => Admd::_1,
        }
    }
    #[doc = "Select mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Admd::_0
    }
    #[doc = "Scan mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Admd::_1
    }
}
#[doc = "Field `ADMD` writer - Specification of the A/D conversion channel selection mode"]
pub type AdmdW<'a, REG> = crate::BitWriter<'a, REG, Admd>;
impl<'a, REG> AdmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Admd::_0)
    }
    #[doc = "Scan mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Admd::_1)
    }
}
#[doc = "A/D conversion operation control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcs {
    #[doc = "0: Stops conversion operation \\[When read\\]
Conversion is stopped or in standby"]
    _0 = 0,
    #[doc = "1: Enables conversion operation \\[When read\\]
While in the no wait mode (both software and hardware trigger mode):Conversion is enabledWhile in the wait mode (both software and hardware trigger mode):A/D power supply stabilization wait time + conversion"]
    _1 = 1,
}
impl From<Adcs> for bool {
    #[inline(always)]
    fn from(variant: Adcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCS` reader - A/D conversion operation control"]
pub type AdcsR = crate::BitReader<Adcs>;
impl AdcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcs {
        match self.bits {
            false => Adcs::_0,
            true => Adcs::_1,
        }
    }
    #[doc = "Stops conversion operation \\[When read\\]
Conversion is stopped or in standby"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adcs::_0
    }
    #[doc = "Enables conversion operation \\[When read\\]
While in the no wait mode (both software and hardware trigger mode):Conversion is enabledWhile in the wait mode (both software and hardware trigger mode):A/D power supply stabilization wait time + conversion"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adcs::_1
    }
}
#[doc = "Field `ADCS` writer - A/D conversion operation control"]
pub type AdcsW<'a, REG> = crate::BitWriter<'a, REG, Adcs>;
impl<'a, REG> AdcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stops conversion operation \\[When read\\]
Conversion is stopped or in standby"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcs::_0)
    }
    #[doc = "Enables conversion operation \\[When read\\]
While in the no wait mode (both software and hardware trigger mode):Conversion is enabledWhile in the wait mode (both software and hardware trigger mode):A/D power supply stabilization wait time + conversion"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcs::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D voltage comparator operation control"]
    #[inline(always)]
    pub fn adce(&self) -> AdceR {
        AdceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Select Operation voltage mode"]
    #[inline(always)]
    pub fn lv(&self) -> LvR {
        LvR::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 3:5 - Select Conversion Clock (fAD)"]
    #[inline(always)]
    pub fn fr(&self) -> FrR {
        FrR::new((self.bits >> 3) & 7)
    }
    #[doc = "Bit 6 - Specification of the A/D conversion channel selection mode"]
    #[inline(always)]
    pub fn admd(&self) -> AdmdR {
        AdmdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A/D conversion operation control"]
    #[inline(always)]
    pub fn adcs(&self) -> AdcsR {
        AdcsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D voltage comparator operation control"]
    #[inline(always)]
    pub fn adce(&mut self) -> AdceW<Adm0Spec> {
        AdceW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Select Operation voltage mode"]
    #[inline(always)]
    pub fn lv(&mut self) -> LvW<Adm0Spec> {
        LvW::new(self, 1)
    }
    #[doc = "Bits 3:5 - Select Conversion Clock (fAD)"]
    #[inline(always)]
    pub fn fr(&mut self) -> FrW<Adm0Spec> {
        FrW::new(self, 3)
    }
    #[doc = "Bit 6 - Specification of the A/D conversion channel selection mode"]
    #[inline(always)]
    pub fn admd(&mut self) -> AdmdW<Adm0Spec> {
        AdmdW::new(self, 6)
    }
    #[doc = "Bit 7 - A/D conversion operation control"]
    #[inline(always)]
    pub fn adcs(&mut self) -> AdcsW<Adm0Spec> {
        AdcsW::new(self, 7)
    }
}
#[doc = "A/D Converter Mode Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adm0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adm0Spec;
impl crate::RegisterSpec for Adm0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adm0::R`](R) reader structure"]
impl crate::Readable for Adm0Spec {}
#[doc = "`write(|w| ..)` method takes [`adm0::W`](W) writer structure"]
impl crate::Writable for Adm0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADM0 to value 0"]
impl crate::Resettable for Adm0Spec {
    const RESET_VALUE: u8 = 0;
}
