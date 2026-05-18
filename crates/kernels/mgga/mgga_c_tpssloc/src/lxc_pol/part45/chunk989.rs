//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 989/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk989<F: Float>(t23035: F, t23241: F, t31366: F, t114790: F, t23164: F, t6555: F, t1880: F, t23237: F, t31419: F, t2047: F, t212: F, t23171: F, t6554: F) -> (F, F, F, F) {
    let t114913 = t23035 * t31366 * t23241;
    let t114916 = t23164 * t114790 * t6555;
    let t114926 = t1880 * t23237 * t31419;
    let t114932 = t23171 * t212 * t2047 * t6554;
    (t114913, t114916, t114926, t114932)
}
