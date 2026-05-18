//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 476/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk476<F: Float>(t3076: F, t352: F, t2044: F, t13839: F, t3077: F, t7501: F, t211: F, t384: F, t1965: F) -> (F, F, F, F, F) {
    let t13840 = t3076 * t352;
    let t13841 = t2044 * t13840;
    let t13842 = t13839 * t13841;
    let t13844 = t7501 * t3077;
    let t13847 = t211 * t384;
    let t13848 = t1965 * t13847;
    (t13841, t13842, t13844, t13847, t13848)
}
