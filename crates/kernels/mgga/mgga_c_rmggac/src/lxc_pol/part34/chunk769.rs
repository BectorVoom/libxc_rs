//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 769/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk769<F: Float>(t446: F, t515: F, t570: F, t14125: F, t68421: F, t2367: F, t3351: F, t498: F, t7231: F, t14117: F, t68448: F, t68455: F, t9045: F) -> (F, F, F, F) {
    let t73889 = t515 * t570 * t446;
    let t73891 = t68421 * t14125 * t73889;
    let t73896 = t3351 * t7231 * t515 * t2367 * t498;
    let t73899 = t68448 * t14117 * t73889;
    let t73902 = t68455 * t14117 * t9045;
    (t73891, t73896, t73899, t73902)
}
