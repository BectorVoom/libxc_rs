//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 419/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk419<F: Float>(t1983: F, t2020: F, t1401: F, t1873: F, t50: F, t56: F, t63: F, t67: F) -> (F, F, F, F) {
    let t2021 = t1983 * t2020;
    let t2028 = 0.135e2 * t1401 * t1873;
    let t2108 = t50 * t56 - t63;
    let t2109 = t2108 * t67;
    (t2021, t2028, t2108, t2109)
}
