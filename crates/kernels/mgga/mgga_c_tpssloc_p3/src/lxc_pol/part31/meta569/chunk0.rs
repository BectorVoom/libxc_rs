//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1801/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1801<F: Float>(t81912: F, t1878: F, t81982: F, t25120: F, t6604: F, t81962: F, t7500: F, t81911: F, t81928: F, t81934: F, t81943: F, t22690: F, t23122: F, t4119: F, t841: F) -> (F, F, F, F, F, F, F, F) {
    let t87414 = F::cast_from(0.22608743412718618878e-1_f64) * t81912;
    let t87420 = t1878 * t81982;
    let t87425 = t81962 * t6604 * t25120;
    let t87432 = t81911 * t7500;
    let t87437 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t81928;
    let t87438 = F::cast_from(0.13565246047631171327e0_f64) * t81934;
    let t87440 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t81943;
    let t87443 = t23122 * t22690 * t841 * t4119;
    (t87414, t87420, t87425, t87432, t87437, t87438, t87440, t87443)
}
