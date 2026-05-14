//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 785/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk785<F: Float>(t14429: F, t14430: F, t14532: F, t14533: F, t14534: F, t14535: F, t14536: F, t14537: F, t14538: F, t14539: F, t14540: F, t14541: F, t14542: F, t14545: F, t14546: F, t15459: F) -> (F,) {
    let t76590 = -t14429 + t15459 + t14430 + t14532 - t14533 + t14534 - t14535 - t14536 + t14537 + t14538 + t14539 - t14540 + t14541 + t14542 + t14545 - t14546;
    (t76590,)
}
