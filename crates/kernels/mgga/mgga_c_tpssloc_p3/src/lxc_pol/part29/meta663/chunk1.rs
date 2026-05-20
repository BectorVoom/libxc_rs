//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2205/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2205<F: Float>(t12407: F, t22633: F, t5335: F, t6976: F, t22704: F, t22705: F, t5345: F, t1992: F, t54918: F, t550: F, t22690: F, t552: F) -> (F, F, F, F) {
    let t90778 = t22633 * t6976 * t5335 * t12407;
    let t90781 = t22704 * t22705 * t5345;
    let t90782 = F::cast_from(0.82246703342411321824e-2_f64) * t90781;
    let t90785 = t1992 * t6976 * t54918 * t550;
    let t90787 = t22690 * t552;
    (t90778, t90782, t90785, t90787)
}
