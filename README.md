# spin-locks

[spin-locks] is a rust crate that provides a Intel hardware-optimized spin lock that uses Hardware Lock Elision (HLE) and a non-CAS based spin lock (an OR lock) as a fast fallback.

These locks do not provide a Mutex or a MutexGuard to automatically unlock on drop, as they do not protect a particular item of data. This is to make it easier to adapt these locks with the use of persistent memory, which is non-stack and non-heap allocated.


## Licensing

The license for this project is MIT.

[spin-locks]: https://github.com/lemonrock/spin-locks "spin-locks GitHub page"
