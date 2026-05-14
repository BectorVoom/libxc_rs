//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 671/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk671<F: Float>(t1251: F, t7301: F, t7300: F, t1190: F, t2144: F, t1193: F, t2127: F, t210: F, t2120: F) -> (F, F, F, F, F) {
    let t7302 = t7301 * t1251;
    let t7303 = t7300 * t7302;
    let t7306 = t1190 * t2144;
    let t7309 = t2127 * t1193 / 288.0;
    let t7310 = t2120 * t210;
    (t7302, t7303, t7306, t7309, t7310)
}
