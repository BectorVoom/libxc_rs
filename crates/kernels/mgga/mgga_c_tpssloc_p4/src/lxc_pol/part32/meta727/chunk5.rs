//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2358/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2358<F: Float>(t104977: F, t1442: F, t1459: F, t19451: F, t20109: F, t24932: F, t27858: F, t27863: F, t27888: F, t29848: F, t4037: F, t4072: F, t4073: F, t5460: F, t650: F, t652: F, t7266: F, t7271: F, t8103: F, t97792: F, t97794: F, t97796: F, t97798: F, t97800: F, t97802: F, t97805: F, t97808: F, t97811: F) -> F {
    let t105045 = -F::cast_from(4.0_f64) * t4072 * t652 * t8103 - F::cast_from(4.0_f64) * t104977 * t1459 - F::cast_from(2.0_f64) * t1442 * t27858 - F::cast_from(2.0_f64) * t19451 * t7271 - F::cast_from(4.0_f64) * t20109 * t7266 - F::cast_from(4.0_f64) * t24932 * t5460 - F::cast_from(4.0_f64) * t27863 * t4037 - F::cast_from(4.0_f64) * t27863 * t4073 - F::cast_from(4.0_f64) * t27888 * t5460 - t29848 * t650 - t97792 + t97794 - t97796 - t97798 - t97800 - t97802 + t97805 - t97808 - t97811;
    t105045
}
