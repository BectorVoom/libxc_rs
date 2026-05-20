//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2208/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2208<F: Float>(t22892: F, t22893: F, t26388: F, t7733: F, t81186: F, t5318: F, t552: F, t1307: F, t6637: F, t6888: F, t1352: F, t22633: F, t6976: F, t90754: F) -> (F, F, F, F) {
    let t90805 = t22892 * t22893 * t26388;
    let t90806 = F::cast_from(0.16449340668482264365e-1_f64) * t90805;
    let t90807 = t81186 * t7733;
    let t90809 = t552 * t5318;
    let t90812 = t6888 * t6637 * t90809 * t1307;
    let t90816 = t22633 * t6976 * t90754 * t1352;
    (t90806, t90807, t90812, t90816)
}
