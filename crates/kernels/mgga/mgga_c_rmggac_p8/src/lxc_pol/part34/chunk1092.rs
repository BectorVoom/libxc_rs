//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1092/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1092<F: Float>(t14685: F, t14689: F, t14690: F, t14691: F, t14692: F, t14693: F, t14694: F, t14695: F, t15613: F, t15615: F, t15619: F, t15620: F, t15623: F, t15628: F, t70741: F) -> F {
    let t78626 = t14685 + t15613 - t15615 - t15619 - t15620 - t15623 - t70741 + t14689 - t14690 - t14691 + t14692 - t14693 - t14694 + t14695 + t15628;
    t78626
}
