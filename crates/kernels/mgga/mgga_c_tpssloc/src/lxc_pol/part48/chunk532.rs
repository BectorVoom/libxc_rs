//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 532/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk532<F: Float>(t1999: F, t6600: F, t6932: F, t1996: F, t6604: F) -> (F, F, F, F) {
    let t6933 = t6600 * t1999;
    let t6934 = t6932 * t6933;
    let t6935 = 0.33643963411783659045e-4 * t6934;
    let t6936 = t1996 * t6604;
    (t6933, t6934, t6935, t6936)
}
