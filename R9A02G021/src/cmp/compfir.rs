#[doc = "Register `COMPFIR` reader"]
pub type R = crate::R<CompfirSpec>;
#[doc = "Register `COMPFIR` writer"]
pub type W = crate::W<CompfirSpec>;
#[doc = "Comparator 0 digital filter selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0fck {
    #[doc = "0: No comparator 0 filter"]
    _00 = 0,
    #[doc = "1: Comparator 0 filter enabled, sampling at PCLKB"]
    _01 = 1,
    #[doc = "2: Comparator 0 filter enabled, sampling at PCLKB/8"]
    _10 = 2,
    #[doc = "3: Comparator 0 filter enabled, sampling at PCLKB/32"]
    _11 = 3,
}
impl From<C0fck> for u8 {
    #[inline(always)]
    fn from(variant: C0fck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0fck {
    type Ux = u8;
}
impl crate::IsEnum for C0fck {}
#[doc = "Field `C0FCK` reader - Comparator 0 digital filter selection"]
pub type C0fckR = crate::FieldReader<C0fck>;
impl C0fckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0fck {
        match self.bits {
            0 => C0fck::_00,
            1 => C0fck::_01,
            2 => C0fck::_10,
            3 => C0fck::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No comparator 0 filter"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == C0fck::_00
    }
    #[doc = "Comparator 0 filter enabled, sampling at PCLKB"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == C0fck::_01
    }
    #[doc = "Comparator 0 filter enabled, sampling at PCLKB/8"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == C0fck::_10
    }
    #[doc = "Comparator 0 filter enabled, sampling at PCLKB/32"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == C0fck::_11
    }
}
#[doc = "Field `C0FCK` writer - Comparator 0 digital filter selection"]
pub type C0fckW<'a, REG> = crate::FieldWriter<'a, REG, 2, C0fck, crate::Safe>;
impl<'a, REG> C0fckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No comparator 0 filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(C0fck::_00)
    }
    #[doc = "Comparator 0 filter enabled, sampling at PCLKB"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(C0fck::_01)
    }
    #[doc = "Comparator 0 filter enabled, sampling at PCLKB/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(C0fck::_10)
    }
    #[doc = "Comparator 0 filter enabled, sampling at PCLKB/32"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(C0fck::_11)
    }
}
#[doc = "Comparator 0 edge polarity switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0epo {
    #[doc = "0: Interrupt request at comparator 0 rising edge"]
    _0 = 0,
    #[doc = "1: Interrupt request at comparator 0 falling edge"]
    _1 = 1,
}
impl From<C0epo> for bool {
    #[inline(always)]
    fn from(variant: C0epo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0EPO` reader - Comparator 0 edge polarity switching"]
pub type C0epoR = crate::BitReader<C0epo>;
impl C0epoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0epo {
        match self.bits {
            false => C0epo::_0,
            true => C0epo::_1,
        }
    }
    #[doc = "Interrupt request at comparator 0 rising edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0epo::_0
    }
    #[doc = "Interrupt request at comparator 0 falling edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0epo::_1
    }
}
#[doc = "Field `C0EPO` writer - Comparator 0 edge polarity switching"]
pub type C0epoW<'a, REG> = crate::BitWriter<'a, REG, C0epo>;
impl<'a, REG> C0epoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request at comparator 0 rising edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0epo::_0)
    }
    #[doc = "Interrupt request at comparator 0 falling edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0epo::_1)
    }
}
#[doc = "Comparator 0 edge detection selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0edg {
    #[doc = "0: Interrupt request by comparator 0 one-edge detection"]
    _0 = 0,
    #[doc = "1: Interrupt request by comparator 0 both-edge detection"]
    _1 = 1,
}
impl From<C0edg> for bool {
    #[inline(always)]
    fn from(variant: C0edg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0EDG` reader - Comparator 0 edge detection selection"]
pub type C0edgR = crate::BitReader<C0edg>;
impl C0edgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0edg {
        match self.bits {
            false => C0edg::_0,
            true => C0edg::_1,
        }
    }
    #[doc = "Interrupt request by comparator 0 one-edge detection"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0edg::_0
    }
    #[doc = "Interrupt request by comparator 0 both-edge detection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0edg::_1
    }
}
#[doc = "Field `C0EDG` writer - Comparator 0 edge detection selection"]
pub type C0edgW<'a, REG> = crate::BitWriter<'a, REG, C0edg>;
impl<'a, REG> C0edgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request by comparator 0 one-edge detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0edg::_0)
    }
    #[doc = "Interrupt request by comparator 0 both-edge detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0edg::_1)
    }
}
#[doc = "Comparator 1 digital filter selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1fck {
    #[doc = "0: No comparator 1 filter"]
    _00 = 0,
    #[doc = "1: Comparator 1 filter enabled, sampling at PCLKB"]
    _01 = 1,
    #[doc = "2: Comparator 1 filter enabled, sampling at PCLKB/8"]
    _10 = 2,
    #[doc = "3: Comparator 1 filter enabled, sampling at PCLKB/32"]
    _11 = 3,
}
impl From<C1fck> for u8 {
    #[inline(always)]
    fn from(variant: C1fck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1fck {
    type Ux = u8;
}
impl crate::IsEnum for C1fck {}
#[doc = "Field `C1FCK` reader - Comparator 1 digital filter selection"]
pub type C1fckR = crate::FieldReader<C1fck>;
impl C1fckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1fck {
        match self.bits {
            0 => C1fck::_00,
            1 => C1fck::_01,
            2 => C1fck::_10,
            3 => C1fck::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No comparator 1 filter"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == C1fck::_00
    }
    #[doc = "Comparator 1 filter enabled, sampling at PCLKB"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == C1fck::_01
    }
    #[doc = "Comparator 1 filter enabled, sampling at PCLKB/8"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == C1fck::_10
    }
    #[doc = "Comparator 1 filter enabled, sampling at PCLKB/32"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == C1fck::_11
    }
}
#[doc = "Field `C1FCK` writer - Comparator 1 digital filter selection"]
pub type C1fckW<'a, REG> = crate::FieldWriter<'a, REG, 2, C1fck, crate::Safe>;
impl<'a, REG> C1fckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No comparator 1 filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(C1fck::_00)
    }
    #[doc = "Comparator 1 filter enabled, sampling at PCLKB"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(C1fck::_01)
    }
    #[doc = "Comparator 1 filter enabled, sampling at PCLKB/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(C1fck::_10)
    }
    #[doc = "Comparator 1 filter enabled, sampling at PCLKB/32"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(C1fck::_11)
    }
}
#[doc = "Comparator 1 edge polarity switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1epo {
    #[doc = "0: Interrupt request at comparator 1 rising edge"]
    _0 = 0,
    #[doc = "1: Interrupt request at comparator 1 falling edge"]
    _1 = 1,
}
impl From<C1epo> for bool {
    #[inline(always)]
    fn from(variant: C1epo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1EPO` reader - Comparator 1 edge polarity switching"]
pub type C1epoR = crate::BitReader<C1epo>;
impl C1epoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1epo {
        match self.bits {
            false => C1epo::_0,
            true => C1epo::_1,
        }
    }
    #[doc = "Interrupt request at comparator 1 rising edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1epo::_0
    }
    #[doc = "Interrupt request at comparator 1 falling edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1epo::_1
    }
}
#[doc = "Field `C1EPO` writer - Comparator 1 edge polarity switching"]
pub type C1epoW<'a, REG> = crate::BitWriter<'a, REG, C1epo>;
impl<'a, REG> C1epoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request at comparator 1 rising edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1epo::_0)
    }
    #[doc = "Interrupt request at comparator 1 falling edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1epo::_1)
    }
}
#[doc = "Comparator 1 edge detection selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1edg {
    #[doc = "0: Interrupt request by comparator 1 one-edge detection"]
    _0 = 0,
    #[doc = "1: Interrupt request by comparator 1 both-edge detection"]
    _1 = 1,
}
impl From<C1edg> for bool {
    #[inline(always)]
    fn from(variant: C1edg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1EDG` reader - Comparator 1 edge detection selection"]
pub type C1edgR = crate::BitReader<C1edg>;
impl C1edgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1edg {
        match self.bits {
            false => C1edg::_0,
            true => C1edg::_1,
        }
    }
    #[doc = "Interrupt request by comparator 1 one-edge detection"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1edg::_0
    }
    #[doc = "Interrupt request by comparator 1 both-edge detection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1edg::_1
    }
}
#[doc = "Field `C1EDG` writer - Comparator 1 edge detection selection"]
pub type C1edgW<'a, REG> = crate::BitWriter<'a, REG, C1edg>;
impl<'a, REG> C1edgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request by comparator 1 one-edge detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1edg::_0)
    }
    #[doc = "Interrupt request by comparator 1 both-edge detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1edg::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparator 0 digital filter selection"]
    #[inline(always)]
    pub fn c0fck(&self) -> C0fckR {
        C0fckR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Comparator 0 edge polarity switching"]
    #[inline(always)]
    pub fn c0epo(&self) -> C0epoR {
        C0epoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator 0 edge detection selection"]
    #[inline(always)]
    pub fn c0edg(&self) -> C0edgR {
        C0edgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Comparator 1 digital filter selection"]
    #[inline(always)]
    pub fn c1fck(&self) -> C1fckR {
        C1fckR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Comparator 1 edge polarity switching"]
    #[inline(always)]
    pub fn c1epo(&self) -> C1epoR {
        C1epoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator 1 edge detection selection"]
    #[inline(always)]
    pub fn c1edg(&self) -> C1edgR {
        C1edgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator 0 digital filter selection"]
    #[inline(always)]
    pub fn c0fck(&mut self) -> C0fckW<CompfirSpec> {
        C0fckW::new(self, 0)
    }
    #[doc = "Bit 2 - Comparator 0 edge polarity switching"]
    #[inline(always)]
    pub fn c0epo(&mut self) -> C0epoW<CompfirSpec> {
        C0epoW::new(self, 2)
    }
    #[doc = "Bit 3 - Comparator 0 edge detection selection"]
    #[inline(always)]
    pub fn c0edg(&mut self) -> C0edgW<CompfirSpec> {
        C0edgW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Comparator 1 digital filter selection"]
    #[inline(always)]
    pub fn c1fck(&mut self) -> C1fckW<CompfirSpec> {
        C1fckW::new(self, 4)
    }
    #[doc = "Bit 6 - Comparator 1 edge polarity switching"]
    #[inline(always)]
    pub fn c1epo(&mut self) -> C1epoW<CompfirSpec> {
        C1epoW::new(self, 6)
    }
    #[doc = "Bit 7 - Comparator 1 edge detection selection"]
    #[inline(always)]
    pub fn c1edg(&mut self) -> C1edgW<CompfirSpec> {
        C1edgW::new(self, 7)
    }
}
#[doc = "Comparator Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compfir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compfir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompfirSpec;
impl crate::RegisterSpec for CompfirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`compfir::R`](R) reader structure"]
impl crate::Readable for CompfirSpec {}
#[doc = "`write(|w| ..)` method takes [`compfir::W`](W) writer structure"]
impl crate::Writable for CompfirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets COMPFIR to value 0"]
impl crate::Resettable for CompfirSpec {
    const RESET_VALUE: u8 = 0;
}
