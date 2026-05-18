//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1033/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1033<F: Float>(t14609: F, t14616: F, t14962: F, t15466: F, t15469: F, t15472: F, t15474: F, t15476: F, t15480: F, t15892: F, t15893: F, t70707: F, t70708: F, t73658: F, t73659: F) -> F {
    let t79953 = t14962 - t14609 - t73658 + t15466 - t15469 - t15472 + t15892 + t15474 + t15893 - t15476 - t15480 + t70707 - t70708 - t73659 - t14616;
    t79953
}
