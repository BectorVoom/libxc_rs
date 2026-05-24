//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 885/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk885<F: Float>(t3080: F, t5267: F, t26291: F, t5888: F, t40724: F, t15075: F, t25441: F, t13819: F, t8358: F, t8362: F, t13823: F, t291: F, t38855: F) -> (F, F, F, F, F, F, F, F) {
    let t75848 = t3080 * t5267;
    let t75850 = F::cast_from(0.17961362552795712846e0_f64) * t26291 * t75848;
    let t75851 = t3080 * t5888;
    let t75853 = F::cast_from(0.17961362552795712846e0_f64) * t40724 * t75851;
    let t75859 = t25441 * t15075;
    let t75864 = t13819 * t8358;
    let t75866 = t13819 * t8362;
    let t75869 = t13823 * t38855 * t291;
    (t75848, t75850, t75851, t75853, t75859, t75864, t75866, t75869)
}
