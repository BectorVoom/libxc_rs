//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2108/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2108<F: Float>(t46953: F, t41466: F, t820: F, t13176: F, t2642: F, t10024: F, t1500: F, t41115: F, t4191: F, t4166: F, t9670: F, t831: F) -> (F, F, F, F, F, F, F) {
    let t46954 = F::new(119.0) / F::new(4608.0) * t46953;
    let t47039 = t41466 * t820;
    let t47044 = t13176 * t2642;
    let t47047 = t1500 * t10024;
    let t47079 = t41115 * t4191;
    let t47080 = F::new(119.0) / F::new(1152.0) * t47079;
    let t47092 = t4166 * t9670;
    let t47093 = t47092 * t831;
    (t46954, t47039, t47044, t47047, t47080, t47092, t47093)
}
