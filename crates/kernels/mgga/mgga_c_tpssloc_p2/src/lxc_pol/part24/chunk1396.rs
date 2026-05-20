//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1396/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1396<F: Float>(t1036: F, t23557: F, t10469: F, t3127: F, t363: F, t1933: F, t1937: F, t2250: F, t3200: F, t83015: F, t1030: F, t1058: F, t3068: F, sigma0: F) -> (F, F, F, F, F) {
    let t83172 = t23557 * t1036;
    let t83196 = t10469 * t3127 * t363;
    let t83206 = t1933 * t2250 * t1937;
    let t83215 = t3200 * t83015;
    let t83220 = t1058 * sigma0 * t1030 * t3068;
    (t83172, t83196, t83206, t83215, t83220)
}
