#[doc = "Register `ITLCSEL0` reader"]
pub type R = crate::R<Itlcsel0Spec>;
#[doc = "Register `ITLCSEL0` writer"]
pub type W = crate::W<Itlcsel0Spec>;
#[doc = "Selection of interval timer count clock (fITL0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Isel {
    #[doc = "0: Counting stops"]
    _000 = 0,
    #[doc = "1: TML32HCLK"]
    _001 = 1,
    #[doc = "2: TML32MOCLK"]
    _010 = 2,
    #[doc = "3: TML32MCLK"]
    _011 = 3,
    #[doc = "4: TML32LCLK/TML32SCLK"]
    _100 = 4,
    #[doc = "5: Event input from the ELC"]
    _101 = 5,
    #[doc = "6: Setting prohibited"]
    Others = 6,
}
impl From<Isel> for u8 {
    #[inline(always)]
    fn from(variant: Isel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Isel {
    type Ux = u8;
}
impl crate::IsEnum for Isel {}
#[doc = "Field `ISEL` reader - Selection of interval timer count clock (fITL0)"]
pub type IselR = crate::FieldReader<Isel>;
impl IselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isel {
        match self.bits {
            0 => Isel::_000,
            1 => Isel::_001,
            2 => Isel::_010,
            3 => Isel::_011,
            4 => Isel::_100,
            5 => Isel::_101,
            _ => Isel::Others,
        }
    }
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Isel::_000
    }
    #[doc = "TML32HCLK"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Isel::_001
    }
    #[doc = "TML32MOCLK"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Isel::_010
    }
    #[doc = "TML32MCLK"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Isel::_011
    }
    #[doc = "TML32LCLK/TML32SCLK"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Isel::_100
    }
    #[doc = "Event input from the ELC"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Isel::_101
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Isel::Others)
    }
}
#[doc = "Field `ISEL` writer - Selection of interval timer count clock (fITL0)"]
pub type IselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Isel, crate::Safe>;
impl<'a, REG> IselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_000)
    }
    #[doc = "TML32HCLK"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_001)
    }
    #[doc = "TML32MOCLK"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_010)
    }
    #[doc = "TML32MCLK"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_011)
    }
    #[doc = "TML32LCLK/TML32SCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_100)
    }
    #[doc = "Event input from the ELC"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_101)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::Others)
    }
}
#[doc = "Selection of interval timer count clock for capturing (fITL1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csel {
    #[doc = "0: Counting stops"]
    _000 = 0,
    #[doc = "1: TML32HCLK"]
    _001 = 1,
    #[doc = "2: TML32MOCLK"]
    _010 = 2,
    #[doc = "3: TML32MCLK"]
    _011 = 3,
    #[doc = "4: TML32LCLK/TML32SCLK"]
    _100 = 4,
    #[doc = "5: Event input from the ELC"]
    _101 = 5,
    #[doc = "6: Setting prohibited"]
    Others = 6,
}
impl From<Csel> for u8 {
    #[inline(always)]
    fn from(variant: Csel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csel {
    type Ux = u8;
}
impl crate::IsEnum for Csel {}
#[doc = "Field `CSEL` reader - Selection of interval timer count clock for capturing (fITL1)"]
pub type CselR = crate::FieldReader<Csel>;
impl CselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csel {
        match self.bits {
            0 => Csel::_000,
            1 => Csel::_001,
            2 => Csel::_010,
            3 => Csel::_011,
            4 => Csel::_100,
            5 => Csel::_101,
            _ => Csel::Others,
        }
    }
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Csel::_000
    }
    #[doc = "TML32HCLK"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Csel::_001
    }
    #[doc = "TML32MOCLK"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Csel::_010
    }
    #[doc = "TML32MCLK"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Csel::_011
    }
    #[doc = "TML32LCLK/TML32SCLK"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Csel::_100
    }
    #[doc = "Event input from the ELC"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Csel::_101
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Csel::Others)
    }
}
#[doc = "Field `CSEL` writer - Selection of interval timer count clock for capturing (fITL1)"]
pub type CselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Csel, crate::Safe>;
impl<'a, REG> CselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counting stops"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Csel::_000)
    }
    #[doc = "TML32HCLK"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Csel::_001)
    }
    #[doc = "TML32MOCLK"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Csel::_010)
    }
    #[doc = "TML32MCLK"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Csel::_011)
    }
    #[doc = "TML32LCLK/TML32SCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Csel::_100)
    }
    #[doc = "Event input from the ELC"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Csel::_101)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Csel::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selection of interval timer count clock (fITL0)"]
    #[inline(always)]
    pub fn isel(&self) -> IselR {
        IselR::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - Selection of interval timer count clock for capturing (fITL1)"]
    #[inline(always)]
    pub fn csel(&self) -> CselR {
        CselR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection of interval timer count clock (fITL0)"]
    #[inline(always)]
    pub fn isel(&mut self) -> IselW<Itlcsel0Spec> {
        IselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Selection of interval timer count clock for capturing (fITL1)"]
    #[inline(always)]
    pub fn csel(&mut self) -> CselW<Itlcsel0Spec> {
        CselW::new(self, 4)
    }
}
#[doc = "Interval Timer Clock Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`itlcsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlcsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itlcsel0Spec;
impl crate::RegisterSpec for Itlcsel0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itlcsel0::R`](R) reader structure"]
impl crate::Readable for Itlcsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`itlcsel0::W`](W) writer structure"]
impl crate::Writable for Itlcsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ITLCSEL0 to value 0"]
impl crate::Resettable for Itlcsel0Spec {
    const RESET_VALUE: u8 = 0;
}
