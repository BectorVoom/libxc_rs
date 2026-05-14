//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1044/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1044<F: Float>(t31137: F, t6907: F, t1985: F, t6891: F, t6888: F, t6920: F, t8462: F, t1307: F, t1998: F, t59: F, t6926: F, t6600: F, t6932: F, t1352: F, t6943: F, t6936: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31138 = t31137 * t6907;
    let t31140 = 0.16449340668482264365e-1 * t1985 * t31138;
    let t31145 = t31137 * t6891;
    let t31147 = 0.3289868133696452873e-1 * t6888 * t31145;
    let t31153 = t6920 * t8462;
    let t31156 = t1998 * t59 * t1307;
    let t31157 = t6926 * t31156;
    let t31159 = t6600 * t8462;
    let t31160 = t6932 * t31159;
    let t31162 = t6943 * t1352;
    let t31163 = t6936 * t31162;
    (t31138, t31140, t31145, t31147, t31153, t31156, t31157, t31159, t31160, t31162, t31163)
}
