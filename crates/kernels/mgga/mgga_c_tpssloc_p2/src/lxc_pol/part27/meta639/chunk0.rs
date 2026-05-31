//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2159/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2159<F: Float>(t7500: F, t81911: F, t81928: F, t81934: F, t81943: F, t22690: F, t23122: F, t4119: F, t841: F, t25064: F, t81902: F, t23077: F, t6646: F) -> (F, F, F, F, F, F, F) {
    let t87432 = t81911 * t7500;
    let t87437 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t81928;
    let t87438 = F::cast_from(0.13565246047631171327e0_f64) * t81934;
    let t87440 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t81943;
    let t87443 = t23122 * t22690 * t841 * t4119;
    let t87444 = F::cast_from(0.40372756094140390854e-3_f64) * t87443;
    let t87445 = t81902 * t25064;
    let t87447 = t23077 * t6646;
    (t87432, t87437, t87438, t87440, t87444, t87445, t87447)
}
