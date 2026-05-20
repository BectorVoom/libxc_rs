//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2507/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2507<F: Float>(t13176: F, t2696: F, t849: F, t13360: F, t2707: F, t1509: F, t9975: F, t242: F, t41347: F, t812: F, t13297: F, t9573: F) -> (F, F, F, F, F, F) {
    let t47278 = t13176 * t2696;
    let t47279 = t47278 * t849;
    let t47283 = t13360 * t2707;
    let t47285 = t1509 * t9975;
    let t47307 = t812 * t41347 * t242;
    let t47333 = t9573 * t13297;
    (t47278, t47279, t47283, t47285, t47307, t47333)
}
