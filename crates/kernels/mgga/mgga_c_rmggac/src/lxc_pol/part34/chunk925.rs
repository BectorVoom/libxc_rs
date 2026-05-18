//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 925/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk925<F: Float>(t14600: F, t14601: F, t14602: F, t14603: F, t14607: F, t14608: F, t14609: F, t15466: F, t15469: F, t15472: F, t15473: F, t15474: F, t15475: F, t15476: F, t70705: F) -> F {
    let t76596 = -t14600 + t14601 + t14602 + t14603 + t14607 + t14608 - t14609 - t70705 + t15466 - t15469 - t15472 + t15473 + t15474 + t15475 - t15476;
    t76596
}
