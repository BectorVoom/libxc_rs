//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2317/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2317<F: Float>(t27604: F, t3523: F, t1232: F, t1748: F, t2132: F, t2136: F, t3587: F, t86129: F, t86228: F, t86248: F, t88391: F, t95446: F, t95450: F, t95452: F, t95456: F, t95459: F, t95463: F) -> F {
    let t95465 = t27604 * t3523 / F::cast_from(324.0_f64);
    let t95469 = t95446 - F::cast_from(0.10093189023535097714e-3_f64) * t2132 * t88391 * t2136 + t95450 / F::cast_from(162.0_f64) + t95452 * t1232 / F::cast_from(216.0_f64) - t95456 + t86228 / F::cast_from(2304.0_f64) - t95459 - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t27604 * t3587 - t95463 + t95465 - t86129 * t1748 / F::cast_from(2304.0_f64) + F::cast_from(0.10093189023535097714e-3_f64) * t86248;
    t95469
}
