//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1236/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1236<F: Float>(t108451: F, t870: F, t105732: F, t105741: F, t105745: F, t105770: F, t105773: F, t105787: F, t105801: F, t105810: F, t1408: F, t1877: F, t20216: F, t2057: F, t2058: F, t24191: F, t25: F, t2522: F, t26563: F, t26756: F, t28241: F, t28249: F, t29106: F, t4314: F, t5397: F, t7114: F, t7475: F, t7845: F, t92319: F) -> (F, F) {
    let t108452 = t108451 * t870;
    let t108466 = F::cast_from(9.0_f64) * t4314 * t2057 * t105810 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t29106 * t1408 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t29106 * t7475 + F::cast_from(3.0_f64) * t26756 * t105770 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t105741 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t105745 - t1877 * t7114 * t105787 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t7845 * t5397 + F::cast_from(3.0_f64) * t105773 * t2058 + F::cast_from(9.0_f64) * t26563 * t105801 + t1877 * t108452 * t25 / F::cast_from(2.0_f64) + F::cast_from(9.0_f64) * t24191 * t105732 + F::cast_from(9.0_f64) * t4314 * t7845 * t28241 - F::cast_from(9.0_f64) * t92319 * t28249 + t1877 * t2057 * t20216 / F::cast_from(2.0_f64);
    (t108452, t108466)
}
