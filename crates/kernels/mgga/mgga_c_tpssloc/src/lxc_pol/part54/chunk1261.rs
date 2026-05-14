//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1261/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1261<F: Float>(t26189: F, t31611: F, t6888: F, t115352: F, t22892: F, t7691: F, t12020: F, t8636: F, t115586: F, t120577: F, t120579: F, t120590: F, t120591: F, t1375: F, t1385: F, t24082: F, t26224: F, t26990: F, t27009: F, t31564: F, t33293: F, t3887: F, t5215: F, t5325: F, t5353: F, t6993: F, t7728: F, t7729: F, t93818: F) -> (F,) {
    let t122328 = t6888 * t31611 * t26189;
    let t122331 = t22892 * t115352 * t7691;
    let t122335 = t12020 * t8636;
    let t122349 = -6.0 * t26224 * t93818 * t7728 + t120577 + 2.0 * t1375 * t3887 * t8636 * t5353 - 0.16449340668482264365e-1 * t122328 + 0.82246703342411321825e-2 * t122331 - 6.0 * t120591 * t26990 + t120579 - 6.0 * t26224 * t122335 * t5325 + 2.0 * t1375 * t3887 * t33293 * t1385 + 2.0 * t5215 * t31564 - t120590 + 2.0 * t24082 * t7729 - 0.82246703342411321824e-2 * t115586 - t27009 * t6993;
    (t122349,)
}
