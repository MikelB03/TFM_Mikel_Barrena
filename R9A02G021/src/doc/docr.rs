#[doc = "Register `DOCR` reader"]
pub type R = crate::R<DocrSpec>;
#[doc = "Register `DOCR` writer"]
pub type W = crate::W<DocrSpec>;
#[doc = "Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oms {
    #[doc = "0: Data comparison mode"]
    _00 = 0,
    #[doc = "1: Data addition mode"]
    _01 = 1,
    #[doc = "2: Data subtraction mode"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Oms> for u8 {
    #[inline(always)]
    fn from(variant: Oms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oms {
    type Ux = u8;
}
impl crate::IsEnum for Oms {}
#[doc = "Field `OMS` reader - Operating Mode Select"]
pub type OmsR = crate::FieldReader<Oms>;
impl OmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oms {
        match self.bits {
            0 => Oms::_00,
            1 => Oms::_01,
            2 => Oms::_10,
            3 => Oms::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Data comparison mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Oms::_00
    }
    #[doc = "Data addition mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Oms::_01
    }
    #[doc = "Data subtraction mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Oms::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Oms::_11
    }
}
#[doc = "Field `OMS` writer - Operating Mode Select"]
pub type OmsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Oms, crate::Safe>;
impl<'a, REG> OmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data comparison mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Oms::_00)
    }
    #[doc = "Data addition mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Oms::_01)
    }
    #[doc = "Data subtraction mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Oms::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Oms::_11)
    }
}
#[doc = "Data Operation Bit Width Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dobw {
    #[doc = "0: 16-bit"]
    _0 = 0,
    #[doc = "1: 32-bit"]
    _1 = 1,
}
impl From<Dobw> for bool {
    #[inline(always)]
    fn from(variant: Dobw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOBW` reader - Data Operation Bit Width Select"]
pub type DobwR = crate::BitReader<Dobw>;
impl DobwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dobw {
        match self.bits {
            false => Dobw::_0,
            true => Dobw::_1,
        }
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dobw::_0
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dobw::_1
    }
}
#[doc = "Field `DOBW` writer - Data Operation Bit Width Select"]
pub type DobwW<'a, REG> = crate::BitWriter<'a, REG, Dobw>;
impl<'a, REG> DobwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dobw::_0)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dobw::_1)
    }
}
#[doc = "Detection Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcsel {
    #[doc = "0: Mismatch (DODSR0 ≠ DODIR)"]
    _000 = 0,
    #[doc = "1: Match (DODSR0 = DODIR)"]
    _001 = 1,
    #[doc = "2: Lower (DODSR0 > DODIR)"]
    _010 = 2,
    #[doc = "3: Upper (DODSR0 < DODIR)"]
    _011 = 3,
    #[doc = "4: Inside window (DODSR0 < DODIR < DODSR1)"]
    _100 = 4,
    #[doc = "5: Outside window (DODIR < DODSR0, DODSR1 < DODIR)"]
    _101 = 5,
    #[doc = "6: Setting prohibited"]
    Others = 6,
}
impl From<Dcsel> for u8 {
    #[inline(always)]
    fn from(variant: Dcsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcsel {
    type Ux = u8;
}
impl crate::IsEnum for Dcsel {}
#[doc = "Field `DCSEL` reader - Detection Condition Select"]
pub type DcselR = crate::FieldReader<Dcsel>;
impl DcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcsel {
        match self.bits {
            0 => Dcsel::_000,
            1 => Dcsel::_001,
            2 => Dcsel::_010,
            3 => Dcsel::_011,
            4 => Dcsel::_100,
            5 => Dcsel::_101,
            _ => Dcsel::Others,
        }
    }
    #[doc = "Mismatch (DODSR0 ≠ DODIR)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dcsel::_000
    }
    #[doc = "Match (DODSR0 = DODIR)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dcsel::_001
    }
    #[doc = "Lower (DODSR0 > DODIR)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dcsel::_010
    }
    #[doc = "Upper (DODSR0 < DODIR)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dcsel::_011
    }
    #[doc = "Inside window (DODSR0 < DODIR < DODSR1)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dcsel::_100
    }
    #[doc = "Outside window (DODIR < DODSR0, DODSR1 < DODIR)"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dcsel::_101
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Dcsel::Others)
    }
}
#[doc = "Field `DCSEL` writer - Detection Condition Select"]
pub type DcselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dcsel, crate::Safe>;
impl<'a, REG> DcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mismatch (DODSR0 ≠ DODIR)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dcsel::_000)
    }
    #[doc = "Match (DODSR0 = DODIR)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dcsel::_001)
    }
    #[doc = "Lower (DODSR0 > DODIR)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dcsel::_010)
    }
    #[doc = "Upper (DODSR0 < DODIR)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dcsel::_011)
    }
    #[doc = "Inside window (DODSR0 < DODIR < DODSR1)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dcsel::_100)
    }
    #[doc = "Outside window (DODIR < DODSR0, DODSR1 < DODIR)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dcsel::_101)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Dcsel::Others)
    }
}
impl R {
    #[doc = "Bits 0:1 - Operating Mode Select"]
    #[inline(always)]
    pub fn oms(&self) -> OmsR {
        OmsR::new(self.bits & 3)
    }
    #[doc = "Bit 3 - Data Operation Bit Width Select"]
    #[inline(always)]
    pub fn dobw(&self) -> DobwR {
        DobwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Detection Condition Select"]
    #[inline(always)]
    pub fn dcsel(&self) -> DcselR {
        DcselR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating Mode Select"]
    #[inline(always)]
    pub fn oms(&mut self) -> OmsW<DocrSpec> {
        OmsW::new(self, 0)
    }
    #[doc = "Bit 3 - Data Operation Bit Width Select"]
    #[inline(always)]
    pub fn dobw(&mut self) -> DobwW<DocrSpec> {
        DobwW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Detection Condition Select"]
    #[inline(always)]
    pub fn dcsel(&mut self) -> DcselW<DocrSpec> {
        DcselW::new(self, 4)
    }
}
#[doc = "DOC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`docr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`docr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DocrSpec;
impl crate::RegisterSpec for DocrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`docr::R`](R) reader structure"]
impl crate::Readable for DocrSpec {}
#[doc = "`write(|w| ..)` method takes [`docr::W`](W) writer structure"]
impl crate::Writable for DocrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DOCR to value 0"]
impl crate::Resettable for DocrSpec {
    const RESET_VALUE: u8 = 0;
}
