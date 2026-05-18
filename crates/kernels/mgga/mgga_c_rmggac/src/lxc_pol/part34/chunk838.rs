//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 838/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk838<F: Float>(t11599: F, t495: F, t14230: F, t2078: F, t3369: F, t14162: F, t8576: F, t14167: F, t14116: F, t14117: F, t9158: F, t9164: F) -> (F, F, F, F) {
    let t75042 = t11599 * t495;
    let t75045 = t14230 * t3369 * t2078 * t75042;
    let t75047 = t8576 * t14162;
    let t75048 = t75047 * t14167;
    let t75051 = t14116 * t14117 * t9158;
    let t75054 = t14116 * t14117 * t9164;
    (t75045, t75048, t75051, t75054)
}
