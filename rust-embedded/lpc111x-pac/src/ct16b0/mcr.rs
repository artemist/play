#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr0i {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr0i> for bool {
    #[inline(always)]
    fn from(variant: Mr0i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0I` reader - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
pub type Mr0iR = crate::BitReader<Mr0i>;
impl Mr0iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr0i {
        match self.bits {
            true => Mr0i::Enabled,
            false => Mr0i::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr0i::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr0i::Disabled
    }
}
#[doc = "Field `MR0I` writer - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
pub type Mr0iW<'a, REG> = crate::BitWriter<'a, REG, Mr0i>;
impl<'a, REG> Mr0iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr0i::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr0i::Disabled)
    }
}
#[doc = "Reset on MR0: the TC will be reset if MR0 matches it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr0r {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr0r> for bool {
    #[inline(always)]
    fn from(variant: Mr0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0R` reader - Reset on MR0: the TC will be reset if MR0 matches it."]
pub type Mr0rR = crate::BitReader<Mr0r>;
impl Mr0rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr0r {
        match self.bits {
            true => Mr0r::Enabled,
            false => Mr0r::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr0r::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr0r::Disabled
    }
}
#[doc = "Field `MR0R` writer - Reset on MR0: the TC will be reset if MR0 matches it."]
pub type Mr0rW<'a, REG> = crate::BitWriter<'a, REG, Mr0r>;
impl<'a, REG> Mr0rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr0r::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr0r::Disabled)
    }
}
#[doc = "Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr0s {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr0s> for bool {
    #[inline(always)]
    fn from(variant: Mr0s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0S` reader - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
pub type Mr0sR = crate::BitReader<Mr0s>;
impl Mr0sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr0s {
        match self.bits {
            true => Mr0s::Enabled,
            false => Mr0s::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr0s::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr0s::Disabled
    }
}
#[doc = "Field `MR0S` writer - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
pub type Mr0sW<'a, REG> = crate::BitWriter<'a, REG, Mr0s>;
impl<'a, REG> Mr0sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr0s::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr0s::Disabled)
    }
}
#[doc = "Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr1i {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr1i> for bool {
    #[inline(always)]
    fn from(variant: Mr1i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1I` reader - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
pub type Mr1iR = crate::BitReader<Mr1i>;
impl Mr1iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr1i {
        match self.bits {
            true => Mr1i::Enabled,
            false => Mr1i::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr1i::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr1i::Disabled
    }
}
#[doc = "Field `MR1I` writer - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
pub type Mr1iW<'a, REG> = crate::BitWriter<'a, REG, Mr1i>;
impl<'a, REG> Mr1iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr1i::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr1i::Disabled)
    }
}
#[doc = "Reset on MR1: the TC will be reset if MR1 matches it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr1r {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr1r> for bool {
    #[inline(always)]
    fn from(variant: Mr1r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1R` reader - Reset on MR1: the TC will be reset if MR1 matches it."]
pub type Mr1rR = crate::BitReader<Mr1r>;
impl Mr1rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr1r {
        match self.bits {
            true => Mr1r::Enabled,
            false => Mr1r::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr1r::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr1r::Disabled
    }
}
#[doc = "Field `MR1R` writer - Reset on MR1: the TC will be reset if MR1 matches it."]
pub type Mr1rW<'a, REG> = crate::BitWriter<'a, REG, Mr1r>;
impl<'a, REG> Mr1rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr1r::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr1r::Disabled)
    }
}
#[doc = "Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr1s {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr1s> for bool {
    #[inline(always)]
    fn from(variant: Mr1s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1S` reader - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
pub type Mr1sR = crate::BitReader<Mr1s>;
impl Mr1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr1s {
        match self.bits {
            true => Mr1s::Enabled,
            false => Mr1s::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr1s::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr1s::Disabled
    }
}
#[doc = "Field `MR1S` writer - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
pub type Mr1sW<'a, REG> = crate::BitWriter<'a, REG, Mr1s>;
impl<'a, REG> Mr1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr1s::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr1s::Disabled)
    }
}
#[doc = "Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr2i {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr2i> for bool {
    #[inline(always)]
    fn from(variant: Mr2i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2I` reader - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
pub type Mr2iR = crate::BitReader<Mr2i>;
impl Mr2iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr2i {
        match self.bits {
            true => Mr2i::Enabled,
            false => Mr2i::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr2i::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr2i::Disabled
    }
}
#[doc = "Field `MR2I` writer - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
pub type Mr2iW<'a, REG> = crate::BitWriter<'a, REG, Mr2i>;
impl<'a, REG> Mr2iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr2i::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr2i::Disabled)
    }
}
#[doc = "Reset on MR2: the TC will be reset if MR2 matches it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr2r {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr2r> for bool {
    #[inline(always)]
    fn from(variant: Mr2r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2R` reader - Reset on MR2: the TC will be reset if MR2 matches it."]
pub type Mr2rR = crate::BitReader<Mr2r>;
impl Mr2rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr2r {
        match self.bits {
            true => Mr2r::Enabled,
            false => Mr2r::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr2r::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr2r::Disabled
    }
}
#[doc = "Field `MR2R` writer - Reset on MR2: the TC will be reset if MR2 matches it."]
pub type Mr2rW<'a, REG> = crate::BitWriter<'a, REG, Mr2r>;
impl<'a, REG> Mr2rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr2r::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr2r::Disabled)
    }
}
#[doc = "Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr2s {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr2s> for bool {
    #[inline(always)]
    fn from(variant: Mr2s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2S` reader - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
pub type Mr2sR = crate::BitReader<Mr2s>;
impl Mr2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr2s {
        match self.bits {
            true => Mr2s::Enabled,
            false => Mr2s::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr2s::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr2s::Disabled
    }
}
#[doc = "Field `MR2S` writer - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
pub type Mr2sW<'a, REG> = crate::BitWriter<'a, REG, Mr2s>;
impl<'a, REG> Mr2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr2s::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr2s::Disabled)
    }
}
#[doc = "Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr3i {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr3i> for bool {
    #[inline(always)]
    fn from(variant: Mr3i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3I` reader - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
pub type Mr3iR = crate::BitReader<Mr3i>;
impl Mr3iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr3i {
        match self.bits {
            true => Mr3i::Enabled,
            false => Mr3i::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr3i::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr3i::Disabled
    }
}
#[doc = "Field `MR3I` writer - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
pub type Mr3iW<'a, REG> = crate::BitWriter<'a, REG, Mr3i>;
impl<'a, REG> Mr3iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr3i::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr3i::Disabled)
    }
}
#[doc = "Reset on MR3: the TC will be reset if MR3 matches it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr3r {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr3r> for bool {
    #[inline(always)]
    fn from(variant: Mr3r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3R` reader - Reset on MR3: the TC will be reset if MR3 matches it."]
pub type Mr3rR = crate::BitReader<Mr3r>;
impl Mr3rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr3r {
        match self.bits {
            true => Mr3r::Enabled,
            false => Mr3r::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr3r::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr3r::Disabled
    }
}
#[doc = "Field `MR3R` writer - Reset on MR3: the TC will be reset if MR3 matches it."]
pub type Mr3rW<'a, REG> = crate::BitWriter<'a, REG, Mr3r>;
impl<'a, REG> Mr3rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr3r::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr3r::Disabled)
    }
}
#[doc = "Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mr3s {
    #[doc = "1: Enabled"]
    Enabled = 1,
    #[doc = "0: Disabled"]
    Disabled = 0,
}
impl From<Mr3s> for bool {
    #[inline(always)]
    fn from(variant: Mr3s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3S` reader - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
pub type Mr3sR = crate::BitReader<Mr3s>;
impl Mr3sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mr3s {
        match self.bits {
            true => Mr3s::Enabled,
            false => Mr3s::Disabled,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mr3s::Enabled
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mr3s::Disabled
    }
}
#[doc = "Field `MR3S` writer - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
pub type Mr3sW<'a, REG> = crate::BitWriter<'a, REG, Mr3s>;
impl<'a, REG> Mr3sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr3s::Enabled)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mr3s::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn mr0i(&self) -> Mr0iR {
        Mr0iR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn mr0r(&self) -> Mr0rR {
        Mr0rR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn mr0s(&self) -> Mr0sR {
        Mr0sR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn mr1i(&self) -> Mr1iR {
        Mr1iR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn mr1r(&self) -> Mr1rR {
        Mr1rR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn mr1s(&self) -> Mr1sR {
        Mr1sR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn mr2i(&self) -> Mr2iR {
        Mr2iR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn mr2r(&self) -> Mr2rR {
        Mr2rR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub fn mr2s(&self) -> Mr2sR {
        Mr2sR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn mr3i(&self) -> Mr3iR {
        Mr3iR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn mr3r(&self) -> Mr3rR {
        Mr3rR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn mr3s(&self) -> Mr3sR {
        Mr3sR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr0i(&mut self) -> Mr0iW<McrSpec> {
        Mr0iW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    #[must_use]
    pub fn mr0r(&mut self) -> Mr0rW<McrSpec> {
        Mr0rW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr0s(&mut self) -> Mr0sW<McrSpec> {
        Mr0sW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr1i(&mut self) -> Mr1iW<McrSpec> {
        Mr1iW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    #[must_use]
    pub fn mr1r(&mut self) -> Mr1rW<McrSpec> {
        Mr1rW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr1s(&mut self) -> Mr1sW<McrSpec> {
        Mr1sW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr2i(&mut self) -> Mr2iW<McrSpec> {
        Mr2iW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    #[must_use]
    pub fn mr2r(&mut self) -> Mr2rW<McrSpec> {
        Mr2rW::new(self, 7)
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr2s(&mut self) -> Mr2sW<McrSpec> {
        Mr2sW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr3i(&mut self) -> Mr3iW<McrSpec> {
        Mr3iW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    #[must_use]
    pub fn mr3r(&mut self) -> Mr3rW<McrSpec> {
        Mr3rW::new(self, 10)
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr3s(&mut self) -> Mr3sW<McrSpec> {
        Mr3sW::new(self, 11)
    }
}
#[doc = "Match Control Register (MCR). The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u32 = 0;
}
