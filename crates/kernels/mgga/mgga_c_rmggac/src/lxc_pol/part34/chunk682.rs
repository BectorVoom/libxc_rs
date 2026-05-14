//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 682/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk682<F: Float>(t15399: F, t69598: F, t21714: F, t68440: F, t9117: F, t3148: F, t3151: F, t38354: F, t21713: F, t68651: F, t9183: F, t14025: F, t35154: F, t9189: F, t9193: F, t9197: F) -> (F, F, F, F, F, F, F) {
    let t74107 = t69598 * t15399;
    let t74112 = t68440 * t21714 * t9117;
    let t74115 = t38354 * t3148 * t3151;
    let t74118 = t21713 * t68651 * t9183;
    let t74120 = t14025 * t35154;
    let t74122 = t21713 * t74120 * t9189;
    let t74125 = t21713 * t21714 * t9193;
    let t74128 = t21713 * t21714 * t9197;
    (t74107, t74112, t74115, t74118, t74122, t74125, t74128)
}
