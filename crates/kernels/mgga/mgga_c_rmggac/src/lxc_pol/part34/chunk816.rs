//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 816/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk816<F: Float>(t15405: F, t7244: F, t3351: F, t498: F, t7231: F, t875: F, t8936: F, t13823: F, t1661: F, t7755: F, t21719: F, t35312: F, t9212: F) -> (F, F, F, F) {
    let t74669 = t7244 * t15405;
    let t74670 = F::new(0.19863479950205658386e-4) * t74669;
    let t74674 = t3351 * t7231 * t875 * t8936 * t498;
    let t74677 = t13823 * t7755 * t1661;
    let t74684 = t21719 * t35312 * t9212;
    (t74670, t74674, t74677, t74684)
}
