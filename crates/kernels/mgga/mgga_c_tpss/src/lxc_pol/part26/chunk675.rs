//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 675/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk675<F: Float>(t2838: F, t4047: F, t128: F, t1289: F, t2845: F, t581: F) -> (F, F, F, F) {
    let t4048 = t2838 * t4047;
    let t4049 = t128 * t4048;
    let t4051 = t2845 * t1289;
    let t4052 = t4051 * t581;
    (t4048, t4049, t4051, t4052)
}
