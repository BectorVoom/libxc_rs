//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 507/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk507<F: Float>(t14504: F, t2228: F, t326: F, t650: F, t699: F, t838: F) -> (F, F, F, F) {
    let t14505 = 0.90915538847484472429e-2 * t14504;
    let t14506 = t326 * t2228;
    let t14507 = t14506 * t650;
    let t14508 = 0.34093327067806677161e-2 * t14507;
    let t14509 = t838 * t699;
    (t14505, t14506, t14508, t14509)
}
