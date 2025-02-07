#[doc = "Register `clicintattr%s` reader"]
pub type R = crate::R<ClicintattrSpec>;
#[doc = "Register `clicintattr%s` writer"]
pub type W = crate::W<ClicintattrSpec>;
#[doc = "Selective Hardware Vectoring type for the associated interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shv {
    #[doc = "0: Non-vectored"]
    _0 = 0,
    #[doc = "1: Vector mode"]
    _1 = 1,
}
impl From<Shv> for bool {
    #[inline(always)]
    fn from(variant: Shv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHV` reader - Selective Hardware Vectoring type for the associated interrupt"]
pub type ShvR = crate::BitReader<Shv>;
impl ShvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shv {
        match self.bits {
            false => Shv::_0,
            true => Shv::_1,
        }
    }
    #[doc = "Non-vectored"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Shv::_0
    }
    #[doc = "Vector mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Shv::_1
    }
}
#[doc = "Field `SHV` writer - Selective Hardware Vectoring type for the associated interrupt"]
pub type ShvW<'a, REG> = crate::BitWriter<'a, REG, Shv>;
impl<'a, REG> ShvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-vectored"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Shv::_0)
    }
    #[doc = "Vector mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Shv::_1)
    }
}
#[doc = "Trigger type for the associated interrupt. This bit is read as 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trg {
    #[doc = "1: Positive Edge"]
    _1 = 1,
}
impl From<Trg> for bool {
    #[inline(always)]
    fn from(variant: Trg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRG` reader - Trigger type for the associated interrupt. This bit is read as 1."]
pub type TrgR = crate::BitReader<Trg>;
impl TrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trg> {
        match self.bits {
            true => Some(Trg::_1),
            _ => None,
        }
    }
    #[doc = "Positive Edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trg::_1
    }
}
#[doc = "Privilege mode that the associated interrupt is triggered. Read value are 2'b11.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "3: Machine"]
    _11 = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Privilege mode that the associated interrupt is triggered. Read value are 2'b11."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            3 => Some(Mode::_11),
            _ => None,
        }
    }
    #[doc = "Machine"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Mode::_11
    }
}
impl R {
    #[doc = "Bit 0 - Selective Hardware Vectoring type for the associated interrupt"]
    #[inline(always)]
    pub fn shv(&self) -> ShvR {
        ShvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger type for the associated interrupt. This bit is read as 1."]
    #[inline(always)]
    pub fn trg(&self) -> TrgR {
        TrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Privilege mode that the associated interrupt is triggered. Read value are 2'b11."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Selective Hardware Vectoring type for the associated interrupt"]
    #[inline(always)]
    pub fn shv(&mut self) -> ShvW<ClicintattrSpec> {
        ShvW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clicintattr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicintattr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClicintattrSpec;
impl crate::RegisterSpec for ClicintattrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clicintattr::R`](R) reader structure"]
impl crate::Readable for ClicintattrSpec {}
#[doc = "`write(|w| ..)` method takes [`clicintattr::W`](W) writer structure"]
impl crate::Writable for ClicintattrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets clicintattr%s to value 0xc2"]
impl crate::Resettable for ClicintattrSpec {
    const RESET_VALUE: u8 = 0xc2;
}
