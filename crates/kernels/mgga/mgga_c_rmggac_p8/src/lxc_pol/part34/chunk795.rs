//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 795/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk795<F: Float>(t21713: F, t68651: F, t9217: F, t68422: F, t9105: F, t9110: F, t236: F, t446: F, t551: F, t21714: F, t68421: F, t15220: F, t7720: F) -> (F, F, F, F, F, F) {
    let t74302 = t21713 * t68651 * t9217;
    let t74305 = t21713 * t68422 * t9105;
    let t74309 = t21713 * t68422 * t9110;
    let t74312 = t236 * t551 * t446;
    let t74314 = t68421 * t21714 * t74312;
    let t74316 = t7720 * t15220;
    (t74302, t74305, t74309, t74312, t74314, t74316)
}
