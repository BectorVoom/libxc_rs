//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 710/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk710<F: Float>(t3154: F, t7921: F, t14040: F, t14367: F, t14042: F, t14115: F, t68454: F, t14147: F, t14151: F, t7348: F, t1295: F, t131: F, t14148: F, t25987: F, t7351: F) -> (F, F, F, F, F, F) {
    let t69835 = F::new(0.66211599834018861287e-4) * t7921 * t3154;
    let t69836 = t14040 * t14367;
    let t69837 = t69836 * t14042;
    let t69838 = F::new(0.20439190441718261718e-5) * t69837;
    let t69839 = t68454 * t14115;
    let t69860 = t14147 * t7348 * t14151;
    let t69865 = t14148 * t7351 * t131 * t1295 * t25987;
    (t69835, t69836, t69838, t69839, t69860, t69865)
}
