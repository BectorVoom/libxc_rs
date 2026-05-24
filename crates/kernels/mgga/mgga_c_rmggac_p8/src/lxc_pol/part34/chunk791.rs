//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 791/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk791<F: Float>(t13819: F, t8346: F, t13823: F, t1665: F, t7755: F, t16503: F, t35039: F, t665: F, t9169: F, t15405: F, t34761: F, t1502: F, t34976: F) -> (F, F, F, F, F) {
    let t74240 = t13819 * t8346;
    let t74243 = t13823 * t7755 * t1665;
    let t74247 = t16503 * t35039 * t665 * t9169;
    let t74249 = t34761 * t15405;
    let t74253 = t16503 * t34976 * t665 * t1502;
    (t74240, t74243, t74247, t74249, t74253)
}
