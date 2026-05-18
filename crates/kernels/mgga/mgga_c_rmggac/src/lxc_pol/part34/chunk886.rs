//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 886/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk886<F: Float>(t14125: F, t21713: F, t8807: F, t1632: F, t3080: F, t26287: F, t1635: F, t26283: F, t5898: F, t26291: F, t5144: F, t30204: F) -> (F, F, F, F, F, F, F, F, F) {
    let t75834 = t21713 * t14125 * t8807;
    let t75836 = t3080 * t1632;
    let t75838 = F::new(0.17961362552795712846e0) * t26287 * t75836;
    let t75839 = t3080 * t1635;
    let t75841 = F::new(0.35922725105591425692e0) * t26283 * t75839;
    let t75842 = t3080 * t5898;
    let t75844 = F::new(0.17961362552795712846e0) * t26291 * t75842;
    let t75845 = t3080 * t5144;
    let t75847 = F::new(0.11974241701863808564e0) * t30204 * t75845;
    (t75834, t75836, t75838, t75839, t75841, t75842, t75844, t75845, t75847)
}
