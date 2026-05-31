//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1087/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1087<F: Float>(t14953: F, t14969: F, t15862: F, t1668: F, t1685: F, t2604: F, t2868: F, t3285: F, t71607: F, t71619: F, t71620: F, t72: F, t75762: F, t75767: F, t77828: F, t78271: F, t78272: F, t78273: F, t78275: F, t78277: F, t78279: F, t78280: F) -> F {
    let t80318 = t77828 + t71607 + t72 * t1685 * t3285 - F::cast_from(0.2363e1_f64) * t1668 * t14953 - F::cast_from(0.59871208509319042821e-1_f64) * t2868 * t14969 + t71619 - t71620 - t78271 - t78272 - t78273 + t78275 - t78277 - t78279 - t78280 - F::cast_from(0.59871208509319042821e-1_f64) * t2604 * t15862 - F::cast_from(0.58171619854173713844e-5_f64) * t75762 - t75767;
    t80318
}
