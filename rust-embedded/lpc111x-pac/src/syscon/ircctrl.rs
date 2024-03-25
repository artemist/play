#[doc = "Register `IRCCTRL` reader"]
pub type R = crate::R<IrcctrlSpec>;
#[doc = "Register `IRCCTRL` writer"]
pub type W = crate::W<IrcctrlSpec>;
#[doc = "Field `TRIM` reader - Trim value"]
pub type TrimR = crate::FieldReader;
#[doc = "Field `TRIM` writer - Trim value"]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Trim value"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trim value"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TrimW<IrcctrlSpec> {
        TrimW::new(self, 0)
    }
}
#[doc = "IRC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ircctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ircctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrcctrlSpec;
impl crate::RegisterSpec for IrcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ircctrl::R`](R) reader structure"]
impl crate::Readable for IrcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ircctrl::W`](W) writer structure"]
impl crate::Writable for IrcctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRCCTRL to value 0x80"]
impl crate::Resettable for IrcctrlSpec {
    const RESET_VALUE: u32 = 0x80;
}
