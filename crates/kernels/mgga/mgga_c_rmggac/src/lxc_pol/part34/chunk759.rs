//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 759/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk759<F: Float>(t14125: F, t68421: F, t73732: F, t352: F, t515: F, t589: F, t68455: F, t9122: F, t68440: F, t9045: F, t21713: F, t68422: F, t9050: F) -> (F, F, F, F, F, F) {
    let t73734 = t68421 * t14125 * t73732;
    let t73737 = t515 * t589 * t352;
    let t73739 = t68421 * t14125 * t73737;
    let t73743 = t68455 * t14125 * t9122;
    let t73746 = t68440 * t14125 * t9045;
    let t73749 = t21713 * t68422 * t9050;
    (t73734, t73737, t73739, t73743, t73746, t73749)
}
