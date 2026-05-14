//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 574/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk574<F: Float>(t1007: F, t34: F, t115: F, t121: F, t859: F, t343: F, t3818: F, t107: F, t837: F) -> (F, F, F, F) {
    let t25561 = 1.0 / t34 / t1007;
    let t25607 = t121 / t859 / t115;
    let t25636 = t343 * t3818;
    let t25640 = t107 * t837;
    (t25561, t25607, t25636, t25640)
}
