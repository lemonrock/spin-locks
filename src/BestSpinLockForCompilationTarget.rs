// This file is part of spin-locks. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/spin-locks/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of spin-locks. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/spin-locks/master/COPYRIGHT.


/// The best spin lock for the compilation target.
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub type BestSpinLockForCompilationTarget = AtomicBoolSpinLock;

/// The best spin lock for the compilation target.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub type BestSpinLockForCompilationTarget = IntelTsxHleSpinLock;
