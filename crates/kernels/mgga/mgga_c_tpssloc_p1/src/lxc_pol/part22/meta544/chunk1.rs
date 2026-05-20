//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2040/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2040<F: Float>(t12289: F, t1336: F, t835: F, t12364: F, t3777: F, t1314: F, t9569: F, t1329: F, t2559: F, t3732: F, t12214: F, t782: F) -> (F, F, F, F, F, F) {
    let t39944 = t1336 * t12289 * t835;
    let t39947 = t3777 * t12364;
    let t40005 = t9569 * t1314;
    let t40006 = t40005 * t1329;
    let t40018 = t2559 * t3732;
    let t40021 = t782 * t12214;
    (t39944, t39947, t40005, t40006, t40018, t40021)
}
