struct MessageInfo {
    inner: u64,
}

impl MessageInfo {
    #[inline(always)]
    fn new() -> Self {
        Self{inner: 0}
    }

    #[inline(always)]
    fn label(&self) -> u64 {
        (self.inner & 0xfffffffffffff000) >> 12
    }

    #[inline(always)]
    fn set_label(self, label: usize) -> <Self, (Self, ())> {
        if label > 0xfffffffffffff { return Err(s, ()); }
        let mut s = self;
        s.inner &= !0xfffffffffffff000;
        s.inner |= ((label as u64) << 12) & 0xfffffffffffff000;
        Ok(s)
    }

    #[inline(always)]
    fn capsUnwrapped(&self) -> u64 {
        (self.inner & 0xe00) >> 9
    }

    #[inline(always)]
    fn set_capsUnwrapped(self, capsUnwrapped: usize) -> <Self, (Self, ())> {
        if capsUnwrapped > 0x7 { return Err(s, ()); }
        let mut s = self;
        s.inner &= !0xe00;
        s.inner |= ((capsUnwrapped as u64) << 9) & 0xe00;
        Ok(s)
    }

    #[inline(always)]
    fn extraCaps(&self) -> u64 {
        (self.inner & 0x180) >> 7
    }

    #[inline(always)]
    fn set_extraCaps(self, extraCaps: usize) -> <Self, (Self, ())> {
        if extraCaps > 0x3 { return Err(s, ()); }
        let mut s = self;
        s.inner &= !0x180;
        s.inner |= ((extraCaps as u64) << 7) & 0x180;
        Ok(s)
    }

    #[inline(always)]
    fn length(&self) -> u64 {
        (self.inner & 0x7f) >> 0
    }

    #[inline(always)]
    fn set_length(self, length: usize) -> <Self, (Self, ())> {
        if length > 0x7f { return Err(s, ()); }
        let mut s = self;
        s.inner &= !0x7f;
        s.inner |= ((length as u64) << 0) & 0x7f;
        Ok(s)
    }
}

struct CapRights {
    inner: u64,
}

impl CapRights {
    #[inline(always)]
    fn new() -> Self {
        Self{inner: 0}
    }

    #[inline(always)]
    fn capAllowGrantReply(&self) -> u64 {
        (self.inner & 0x8) >> 3
    }

    #[inline(always)]
    fn set_capAllowGrantReply(self, capAllowGrantReply: usize) -> <Self, (Self, ())> {
        if capAllowGrantReply > 0x1 { return Err(s, ()); }
        let mut s = self;
        s.inner &= !0x8;
        s.inner |= ((capAllowGrantReply as u64) << 3) & 0x8;
        Ok(s)
    }

    #[inline(always)]
    fn capAllowGrant(&self) -> u64 {
        (self.inner & 0x4) >> 2
    }

    #[inline(always)]
    fn set_capAllowGrant(self, capAllowGrant: usize) -> <Self, (Self, ())> {
        if capAllowGrant > 0x1 { return Err(s, ()); }
        let mut s = self;
        s.inner &= !0x4;
        s.inner |= ((capAllowGrant as u64) << 2) & 0x4;
        Ok(s)
    }

    #[inline(always)]
    fn capAllowRead(&self) -> u64 {
        (self.inner & 0x2) >> 1
    }

    #[inline(always)]
    fn set_capAllowRead(self, capAllowRead: usize) -> <Self, (Self, ())> {
        if capAllowRead > 0x1 { return Err(s, ()); }
        let mut s = self;
        s.inner &= !0x2;
        s.inner |= ((capAllowRead as u64) << 1) & 0x2;
        Ok(s)
    }

    #[inline(always)]
    fn capAllowWrite(&self) -> u64 {
        (self.inner & 0x1) >> 0
    }

    #[inline(always)]
    fn set_capAllowWrite(self, capAllowWrite: usize) -> <Self, (Self, ())> {
        if capAllowWrite > 0x1 { return Err(s, ()); }
        let mut s = self;
        s.inner &= !0x1;
        s.inner |= ((capAllowWrite as u64) << 0) & 0x1;
        Ok(s)
    }
}

struct CNode_CapData {
    inner: u64,
}

impl CNode_CapData {
    #[inline(always)]
    fn new() -> Self {
        Self{inner: 0}
    }

    #[inline(always)]
    fn guard(&self) -> u64 {
        (self.inner & 0xffffffffffffffc0) >> 6
    }

    #[inline(always)]
    fn set_guard(self, guard: usize) -> <Self, (Self, ())> {
        if guard > 0x3ffffffffffffff { return Err(s, ()); }
        let mut s = self;
        s.inner &= !0xffffffffffffffc0;
        s.inner |= ((guard as u64) << 6) & 0xffffffffffffffc0;
        Ok(s)
    }

    #[inline(always)]
    fn guardSize(&self) -> u64 {
        (self.inner & 0x3f) >> 0
    }

    #[inline(always)]
    fn set_guardSize(self, guardSize: usize) -> <Self, (Self, ())> {
        if guardSize > 0x3f { return Err(s, ()); }
        let mut s = self;
        s.inner &= !0x3f;
        s.inner |= ((guardSize as u64) << 0) & 0x3f;
        Ok(s)
    }
}