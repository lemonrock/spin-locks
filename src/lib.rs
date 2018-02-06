// This file is part of spin-locks. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/spin-locks/master/COPYRIGHT. No part of spin-locks, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of spin-locks. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/spin-locks/master/COPYRIGHT.


#![deny(missing_docs)]


//! #spin-locks
//! An Intel hardware-optimized spin lock that uses Hardware Lock Elision (HLE) and a non-CAS based spin lock (an OR lock) as a fast fallback.
//! The intel spin lock, `IntelTsxHleSpinLock`, is only available on x86 and x86_64 targets.
//! To pick the best spin lock for the compilation target, use the type alias `BestSpinLockForCompilationTarget`.
//!


extern crate intel_tsx_hle;


use ::intel_tsx_hle::__hle_acquire_exchange_n1;
use ::intel_tsx_hle::__hle_release_store_n1;
use ::std::cell::UnsafeCell;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering::Acquire;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::Release;
use ::std::sync::atomic::spin_loop_hint;


include!("AtomicBoolSpinLock.rs");
include!("BestSpinLockForCompilationTarget.rs");
include!("IntelTsxHleSpinLock.rs");
include!("SpinLock.rs");
