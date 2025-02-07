#[doc = "Register `TT0` reader"]
pub type R = crate::R<Tt0Spec>;
#[doc = "Register `TT0` writer"]
pub type W = crate::W<Tt0Spec>;
#[doc = "Operation stop trigger of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tt {
    #[doc = "0: No trigger operation"]
    _0 = 0,
    #[doc = "1: The TE0.TE\\[n\\]
bit is cleared to 0 and the count operation is stopped"]
    _1 = 1,
}
impl From<Tt> for u8 {
    #[inline(always)]
    fn from(variant: Tt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tt {
    type Ux = u8;
}
impl crate::IsEnum for Tt {}
#[doc = "Field `TT` reader - Operation stop trigger of channel n"]
pub type TtR = crate::FieldReader<Tt>;
impl TtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tt> {
        match self.bits {
            0 => Some(Tt::_0),
            1 => Some(Tt::_1),
            _ => None,
        }
    }
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tt::_0
    }
    #[doc = "The TE0.TE\\[n\\]
bit is cleared to 0 and the count operation is stopped"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tt::_1
    }
}
#[doc = "Field `TT` writer - Operation stop trigger of channel n"]
pub type TtW<'a, REG> = crate::FieldWriter<'a, REG, 8, Tt>;
impl<'a, REG> TtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tt::_0)
    }
    #[doc = "The TE0.TE\\[n\\]
bit is cleared to 0 and the count operation is stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tt::_1)
    }
}
#[doc = "Trigger to stop operation of the higher 8-bit timer when channel 1 is in the 8-bit timer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tth1 {
    #[doc = "0: No trigger operation"]
    _0 = 0,
    #[doc = "1: The TE0.TEH1 bit is cleared to 0 and the count operation is stopped"]
    _1 = 1,
}
impl From<Tth1> for bool {
    #[inline(always)]
    fn from(variant: Tth1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTH1` reader - Trigger to stop operation of the higher 8-bit timer when channel 1 is in the 8-bit timer mode"]
pub type Tth1R = crate::BitReader<Tth1>;
impl Tth1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tth1 {
        match self.bits {
            false => Tth1::_0,
            true => Tth1::_1,
        }
    }
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tth1::_0
    }
    #[doc = "The TE0.TEH1 bit is cleared to 0 and the count operation is stopped"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tth1::_1
    }
}
#[doc = "Field `TTH1` writer - Trigger to stop operation of the higher 8-bit timer when channel 1 is in the 8-bit timer mode"]
pub type Tth1W<'a, REG> = crate::BitWriter<'a, REG, Tth1>;
impl<'a, REG> Tth1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tth1::_0)
    }
    #[doc = "The TE0.TEH1 bit is cleared to 0 and the count operation is stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tth1::_1)
    }
}
#[doc = "Trigger to stop operation of the higher 8-bit timer when channel 3 is in the 8-bit timer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tth3 {
    #[doc = "0: No trigger operation"]
    _0 = 0,
    #[doc = "1: The TE0.TEH3 bit is cleared to 0 and the count operation is stopped"]
    _1 = 1,
}
impl From<Tth3> for bool {
    #[inline(always)]
    fn from(variant: Tth3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTH3` reader - Trigger to stop operation of the higher 8-bit timer when channel 3 is in the 8-bit timer mode"]
pub type Tth3R = crate::BitReader<Tth3>;
impl Tth3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tth3 {
        match self.bits {
            false => Tth3::_0,
            true => Tth3::_1,
        }
    }
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tth3::_0
    }
    #[doc = "The TE0.TEH3 bit is cleared to 0 and the count operation is stopped"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tth3::_1
    }
}
#[doc = "Field `TTH3` writer - Trigger to stop operation of the higher 8-bit timer when channel 3 is in the 8-bit timer mode"]
pub type Tth3W<'a, REG> = crate::BitWriter<'a, REG, Tth3>;
impl<'a, REG> Tth3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tth3::_0)
    }
    #[doc = "The TE0.TEH3 bit is cleared to 0 and the count operation is stopped"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tth3::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Operation stop trigger of channel n"]
    #[inline(always)]
    pub fn tt(&self) -> TtR {
        TtR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - Trigger to stop operation of the higher 8-bit timer when channel 1 is in the 8-bit timer mode"]
    #[inline(always)]
    pub fn tth1(&self) -> Tth1R {
        Tth1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Trigger to stop operation of the higher 8-bit timer when channel 3 is in the 8-bit timer mode"]
    #[inline(always)]
    pub fn tth3(&self) -> Tth3R {
        Tth3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Operation stop trigger of channel n"]
    #[inline(always)]
    pub fn tt(&mut self) -> TtW<Tt0Spec> {
        TtW::new(self, 0)
    }
    #[doc = "Bit 9 - Trigger to stop operation of the higher 8-bit timer when channel 1 is in the 8-bit timer mode"]
    #[inline(always)]
    pub fn tth1(&mut self) -> Tth1W<Tt0Spec> {
        Tth1W::new(self, 9)
    }
    #[doc = "Bit 11 - Trigger to stop operation of the higher 8-bit timer when channel 3 is in the 8-bit timer mode"]
    #[inline(always)]
    pub fn tth3(&mut self) -> Tth3W<Tt0Spec> {
        Tth3W::new(self, 11)
    }
}
#[doc = "Timer Channel Stop Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tt0Spec;
impl crate::RegisterSpec for Tt0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tt0::R`](R) reader structure"]
impl crate::Readable for Tt0Spec {}
#[doc = "`write(|w| ..)` method takes [`tt0::W`](W) writer structure"]
impl crate::Writable for Tt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TT0 to value 0"]
impl crate::Resettable for Tt0Spec {
    const RESET_VALUE: u16 = 0;
}
