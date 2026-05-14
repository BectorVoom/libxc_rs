//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 407/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk407<F: Float>(t1915: F, t25: F, t1877: F, t365: F, t335: F, t371: F) -> (F, F, F, F) {
    let t1916 = t1915 * t25;
    let t1918 = t1877 * t1916 / 2.0;
    let t1929 = t365 * t365;
    let t1932 = 1.0 / t371 / t335;
    (t1916, t1918, t1929, t1932)
}
