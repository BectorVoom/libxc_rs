//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 761/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk761<F: Float>(t26287: F, t75836: F, t1635: F, t3080: F, t26283: F, t5898: F, t26291: F, t5144: F, t30204: F, t5267: F, t5888: F, t40724: F, t15075: F, t25441: F, t13819: F, t8358: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t75838 = 0.17961362552795712846e0 * t26287 * t75836;
    let t75839 = t3080 * t1635;
    let t75841 = 0.35922725105591425692e0 * t26283 * t75839;
    let t75842 = t3080 * t5898;
    let t75844 = 0.17961362552795712846e0 * t26291 * t75842;
    let t75845 = t3080 * t5144;
    let t75847 = 0.11974241701863808564e0 * t30204 * t75845;
    let t75848 = t3080 * t5267;
    let t75850 = 0.17961362552795712846e0 * t26291 * t75848;
    let t75851 = t3080 * t5888;
    let t75853 = 0.17961362552795712846e0 * t40724 * t75851;
    let t75859 = t25441 * t15075;
    let t75864 = t13819 * t8358;
    (t75838, t75839, t75841, t75842, t75844, t75845, t75847, t75848, t75850, t75851, t75853, t75859, t75864)
}
