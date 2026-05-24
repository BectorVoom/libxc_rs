//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 898/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk898<F: Float>(t1986: F, t2398: F, t7720: F, t17695: F, t511: F, t7231: F, t14258: F, t11674: F, t495: F, t14230: F, t14243: F, t2067: F) -> (F, F, F) {
    let t76089 = t1986 * t2398;
    let t76090 = t7720 * t76089;
    let t76101 = t511 * t17695;
    let t76102 = t7231 * t76101;
    let t76103 = t14258 * t76102;
    let t76105 = t11674 * t495;
    let t76108 = t14230 * t14243 * t2067 * t76105;
    (t76090, t76103, t76108)
}
