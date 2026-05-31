//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1428/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1428<F: Float>(t12339: F, t1831: F, t3866: F, t5314: F, t3865: F, t5234: F, t1369: F, t12189: F, t1811: F, t1358: F, t5231: F, t1815: F, t3862: F) -> (F, F, F, F, F, F, F) {
    let t16325 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t12339 * t1831;
    let t16331 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t3866 * t5314;
    let t16336 = t5234 * t3865;
    let t16338 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t16336 * t1369;
    let t16341 = t12189 * t1811;
    let t16346 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t5231 * t1358;
    let t16350 = t1815 * t3862;
    (t16325, t16331, t16336, t16338, t16341, t16346, t16350)
}
