//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 554/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk554<F: Float>(t14461: F, t14471: F, t14505: F, t15086: F, t15089: F, t15092: F, t15140: F, t15142: F, t15146: F, t15535: F, t15538: F, t15541: F, t15544: F, t15545: F, t15546: F, t15549: F, t15550: F, t15551: F, t15552: F, t15557: F) -> (F,) {
    let t15558 = t15535 - t15538 + t15541 + t15086 - t15089 + t15092 + t15544 - t15545 + t15546 - t14461 + t14471 + t15549 - t14505 + t15550 - t15551 - t15552 - 0.93188427318671584245e-2 * t15140 + 0.15531404553111930708e-1 * t15142 + 0.31062809106223861415e-2 * t15146 - t15557;
    (t15558,)
}
