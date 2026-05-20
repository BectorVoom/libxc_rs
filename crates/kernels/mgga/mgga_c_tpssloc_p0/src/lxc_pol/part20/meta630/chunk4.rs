//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2288/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2288<F: Float>(t1509: F, t2631: F, t13360: F, t2703: F, t1516: F, t41052: F, t40961: F, t4261: F, t9993: F, t4166: F, t9600: F, t849: F) -> (F, F, F, F, F, F) {
    let t47262 = t1509 * t2631;
    let t47267 = t13360 * t2703;
    let t47269 = t41052 * t1516;
    let t47270 = F::new(119.0) / F::new(1152.0) * t47269;
    let t47271 = t40961 * t1516;
    let t47273 = t9993 * t4261;
    let t47275 = t4166 * t9600;
    let t47276 = t47275 * t849;
    (t47262, t47267, t47270, t47271, t47273, t47276)
}
