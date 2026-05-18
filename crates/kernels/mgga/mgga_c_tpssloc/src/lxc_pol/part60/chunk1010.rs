//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1010/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1010<F: Float>(t1888: F, t23270: F, t26728: F, t5636: F, t121401: F, t1880: F, t7488: F, t101551: F, t113038: F, t113045: F, t121749: F, t121753: F, t126518: F, t126520: F, t126521: F, t1492: F, t17052: F, t17090: F, t17092: F, t25168: F, t259: F, t2718: F, t28306: F, t29091: F, t33395: F, t5657: F, t6627: F, t7516: F, t855: F, t8553: F, t8562: F, t8563: F) -> F {
    let t128049 = t1888 * t23270 * t26728 * t5636;
    let t128070 = t1880 * t121401 * t7488;
    let t128072 = -t17090 * t8563 + t113038 + F::new(2.0) * t1492 * t33395 * t259 - F::new(0.49348022005446793095e-1) * t128049 - F::new(12.0) * t25168 * t26728 * t28306 - t113045 - F::new(12.0) * t25168 * t101551 * t7516 + t126518 + F::new(2.0) * t17052 * t8553 + F::new(4.0) * t17092 * t8553 - F::new(0.82246703342411321824e-2) * t121749 + F::new(0.82246703342411321824e-2) * t121753 + F::new(2.0) * t855 * t2718 * t8562 * t5657 - F::new(6.0) * t6627 * t29091 - t126520 - F::new(0.16449340668482264365e-1) * t128070 + t126521;
    t128072
}
