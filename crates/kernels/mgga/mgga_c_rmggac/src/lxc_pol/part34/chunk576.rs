//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 576/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk576<F: Float>(t209: F, t476: F, t698: F, t515: F, t1971: F, t1970: F, t14469: F, t739: F, t14227: F, t14234: F, t14241: F, t14246: F) -> (F, F, F, F, F, F, F) {
    let t14681 = t698 * t476 * t209;
    let t14682 = t515 * t14681;
    let t14683 = t1971 * t14682;
    let t14684 = t1970 * t14683;
    let t14685 = F::new(0.42564599893297839398e-5) * t14684;
    let t14686 = t739 * t14469;
    let t14689 = F::new(0.1276937996798935182e-4) * t14227;
    let t14690 = F::new(0.72714524817717142308e-5) * t14234;
    let t14691 = F::new(0.58171619854173713846e-5) * t14241;
    let t14692 = F::new(0.17451485956252114154e-4) * t14246;
    (t14683, t14685, t14686, t14689, t14690, t14691, t14692)
}
