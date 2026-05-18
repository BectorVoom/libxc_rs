//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1260/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1260<F: Float>(t26135: F, t7266: F, t104977: F, t1873: F, t27863: F, t6534: F, t122917: F, t96238: F, t122920: F, t33690: F, t31918: F, t4028: F) -> (F, F, F, F, F, F, F, F) {
    let t123054 = t7266 * t26135;
    let t123056 = t104977 * t1873;
    let t123058 = t27863 * t6534;
    let t123060 = t122917 * t1873;
    let t123072 = t96238 * t1873;
    let t123084 = t122920 * t1873;
    let t123086 = t33690 * t6534;
    let t123091 = t4028 * t31918;
    (t123054, t123056, t123058, t123060, t123072, t123084, t123086, t123091)
}
