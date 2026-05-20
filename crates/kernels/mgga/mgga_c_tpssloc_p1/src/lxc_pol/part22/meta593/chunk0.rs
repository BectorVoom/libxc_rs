//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2109/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2109<F: Float>(t47093: F, t4159: F, t9541: F, t1516: F, t41052: F, t4166: F, t9600: F, t849: F, t13176: F, t2696: F, t1509: F, t9975: F) -> (F, F, F, F, F, F, F) {
    let t47094 = F::new(119.0) / F::new(4608.0) * t47093;
    let t47230 = t9541 * t4159;
    let t47231 = F::new(35.0) / F::new(72.0) * t47230;
    let t47269 = t41052 * t1516;
    let t47270 = F::new(119.0) / F::new(1152.0) * t47269;
    let t47275 = t4166 * t9600;
    let t47276 = t47275 * t849;
    let t47277 = F::new(119.0) / F::new(1152.0) * t47276;
    let t47278 = t13176 * t2696;
    let t47285 = t1509 * t9975;
    (t47094, t47231, t47270, t47275, t47277, t47278, t47285)
}
