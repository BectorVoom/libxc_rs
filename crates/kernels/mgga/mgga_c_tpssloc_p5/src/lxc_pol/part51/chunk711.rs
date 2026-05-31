//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 711/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk711<F: Float>(t1929: F, t1932: F, t1934: F, t1933: F, t40: F, t1937: F, t3: F, t607: F, t343: F, t984: F, t1948: F, t363: F, rho0: F) -> (F, F, F, F, F, F, F, F) {
    let t6720 = t1929 * rho0;
    let t6721 = F::cast_from(1.0_f64) / t6720;
    let t6722 = t6721 * t1932;
    let t6723 = t6722 * t1934;
    let t6726 = t1933 * t40;
    let t6728 = F::cast_from(0.10093189023535097714e-3_f64) * t6726 * t1937;
    let t6729 = t3 * t607;
    let t6730 = t1933 * t6729;
    let t6733 = t984 * t343;
    let t6734 = t1948 * t363;
    (t6721, t6722, t6723, t6728, t6729, t6730, t6733, t6734)
}
