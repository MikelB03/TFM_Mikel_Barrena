#[doc = "Register `TS0` reader"]
pub type R = crate::R<Ts0Spec>;
#[doc = "Register `TS0` writer"]
pub type W = crate::W<Ts0Spec>;
#[doc = "Operation enable (start) trigger of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ts {
    #[doc = "0: No trigger operation"]
    _0 = 0,
    #[doc = "1: The TE0.TE\\[n\\]
bit is set to 1 and the count operation becomes enabled"]
    _1 = 1,
}
impl From<Ts> for u8 {
    #[inline(always)]
    fn from(variant: Ts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ts {
    type Ux = u8;
}
impl crate::IsEnum for Ts {}
#[doc = "Field `TS` reader - Operation enable (start) trigger of channel n"]
pub type TsR = crate::FieldReader<Ts>;
impl TsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ts> {
        match self.bits {
            0 => Some(Ts::_0),
            1 => Some(Ts::_1),
            _ => None,
        }
    }
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ts::_0
    }
    #[doc = "The TE0.TE\\[n\\]
bit is set to 1 and the count operation becomes enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ts::_1
    }
}
#[doc = "Field `TS` writer - Operation enable (start) trigger of channel n"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ts>;
impl<'a, REG> TsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::_0)
    }
    #[doc = "The TE0.TE\\[n\\]
bit is set to 1 and the count operation becomes enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::_1)
    }
}
#[doc = "Trigger to enable operation (start operation) of the higher 8-bit timer when channel 1 is in the 8-bit timer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsh1 {
    #[doc = "0: No trigger operation"]
    _0 = 0,
    #[doc = "1: The TE0.TEH1 bit is set to 1 and the count operation becomes enabled"]
    _1 = 1,
}
impl From<Tsh1> for bool {
    #[inline(always)]
    fn from(variant: Tsh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSH1` reader - Trigger to enable operation (start operation) of the higher 8-bit timer when channel 1 is in the 8-bit timer mode"]
pub type Tsh1R = crate::BitReader<Tsh1>;
impl Tsh1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsh1 {
        match self.bits {
            false => Tsh1::_0,
            true => Tsh1::_1,
        }
    }
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsh1::_0
    }
    #[doc = "The TE0.TEH1 bit is set to 1 and the count operation becomes enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsh1::_1
    }
}
#[doc = "Field `TSH1` writer - Trigger to enable operation (start operation) of the higher 8-bit timer when channel 1 is in the 8-bit timer mode"]
pub type Tsh1W<'a, REG> = crate::BitWriter<'a, REG, Tsh1>;
impl<'a, REG> Tsh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsh1::_0)
    }
    #[doc = "The TE0.TEH1 bit is set to 1 and the count operation becomes enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsh1::_1)
    }
}
#[doc = "Trigger to enable operation (start operation) of the higher 8-bit timer when channel 3 is in the 8-bit timer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsh3 {
    #[doc = "0: No trigger operation"]
    _0 = 0,
    #[doc = "1: The TE0.TEH3 bit is set to 1 and the count operation becomes enabled"]
    _1 = 1,
}
impl From<Tsh3> for bool {
    #[inline(always)]
    fn from(variant: Tsh3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSH3` reader - Trigger to enable operation (start operation) of the higher 8-bit timer when channel 3 is in the 8-bit timer mode"]
pub type Tsh3R = crate::BitReader<Tsh3>;
impl Tsh3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsh3 {
        match self.bits {
            false => Tsh3::_0,
            true => Tsh3::_1,
        }
    }
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsh3::_0
    }
    #[doc = "The TE0.TEH3 bit is set to 1 and the count operation becomes enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsh3::_1
    }
}
#[doc = "Field `TSH3` writer - Trigger to enable operation (start operation) of the higher 8-bit timer when channel 3 is in the 8-bit timer mode"]
pub type Tsh3W<'a, REG> = crate::BitWriter<'a, REG, Tsh3>;
impl<'a, REG> Tsh3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsh3::_0)
    }
    #[doc = "The TE0.TEH3 bit is set to 1 and the count operation becomes enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsh3::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Operation enable (start) trigger of channel n"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - Trigger to enable operation (start operation) of the higher 8-bit timer when channel 1 is in the 8-bit timer mode"]
    #[inline(always)]
    pub fn tsh1(&self) -> Tsh1R {
        Tsh1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Trigger to enable operation (start operation) of the higher 8-bit timer when channel 3 is in the 8-bit timer mode"]
    #[inline(always)]
    pub fn tsh3(&self) -> Tsh3R {
        Tsh3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Operation enable (start) trigger of channel n"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<Ts0Spec> {
        TsW::new(self, 0)
    }
    #[doc = "Bit 9 - Trigger to enable operation (start operation) of the higher 8-bit timer when channel 1 is in the 8-bit timer mode"]
    #[inline(always)]
    pub fn tsh1(&mut self) -> Tsh1W<Ts0Spec> {
        Tsh1W::new(self, 9)
    }
    #[doc = "Bit 11 - Trigger to enable operation (start operation) of the higher 8-bit timer when channel 3 is in the 8-bit timer mode"]
    #[inline(always)]
    pub fn tsh3(&mut self) -> Tsh3W<Ts0Spec> {
        Tsh3W::new(self, 11)
    }
}
#[doc = "Timer Channel Start Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ts0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts0Spec;
impl crate::RegisterSpec for Ts0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ts0::R`](R) reader structure"]
impl crate::Readable for Ts0Spec {}
#[doc = "`write(|w| ..)` method takes [`ts0::W`](W) writer structure"]
impl crate::Writable for Ts0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TS0 to value 0"]
impl crate::Resettable for Ts0Spec {
    const RESET_VALUE: u16 = 0;
}
