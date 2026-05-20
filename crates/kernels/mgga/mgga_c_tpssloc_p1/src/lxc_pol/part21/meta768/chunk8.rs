//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2661/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2661<F: Float>(t19297: F, t604: F, t4021: F, t12571: F, t12585: F, t12588: F, t19299: F, t19310: F, t19318: F, t19445: F, t2235: F, t2240: F, t2241: F, t2307: F, t39054: F, t39063: F, t3958: F, t46104: F, t5389: F, t5445: F, t55631: F, t55673: F, t55709: F, t55875: F, t605: F, t645: F, t9228: F, t9231: F, t9239: F) -> F {
    let t55880 = t19297 * t604;
    let t55885 = t4021 * t4021;
    let t55888 = F::new(40.0) * t9231 * t19318 - F::new(240.0) * t39054 * t19310 - F::new(120.0) * t9239 * t5389 * t2307 - F::new(8.0) * t2235 * t19445 + F::new(80.0) * t46104 * t3958 + F::new(80.0) * t12571 * t12585 + F::new(40.0) * t12571 * t12588 + F::new(840.0) * t39063 * t5389 * t2241 - F::new(4.0) * t9228 * t5445 - F::new(4.0) * t605 * (t55631 + t55673 + t55709 + t55875) - F::new(8.0) * t55880 * t645 - F::new(4.0) * t19299 * t2307 + F::new(40.0) * t2240 * t55885;
    t55888
}
