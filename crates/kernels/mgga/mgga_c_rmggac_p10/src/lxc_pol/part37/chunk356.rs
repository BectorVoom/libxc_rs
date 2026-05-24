//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 356/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk356<F: Float>(t325: F, t623: F, t108: F, t1539: F, t117: F, t3807: F, t1679: F, t107: F, t622: F) -> (F, F, F, F, F) {
    let t4985 = t623 * t325;
    let t5011 = t1539 * t108;
    let t5016 = t3807 * t117;
    let t5055 = t1679 * t325;
    let t5058 = t622 * t107;
    (t4985, t5011, t5016, t5055, t5058)
}
