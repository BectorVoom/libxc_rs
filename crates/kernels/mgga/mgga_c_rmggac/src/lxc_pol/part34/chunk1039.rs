//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1039/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1039<F: Float>(t76244: F, t76246: F, t76250: F, t76253: F, t77963: F, t77966: F, t77969: F, t77973: F, t77976: F, t77979: F, t77982: F, t77983: F, t77988: F) -> F {
    let t77989 = -t77963 + t77966 + t77969 + t77973 + t77976 + t77979 - t77982 + t77983 - F::cast_from(0.93188427318671584245e-2_f64) * t76244 + F::cast_from(0.15531404553111930708e-1_f64) * t76246 + F::cast_from(0.31062809106223861415e-2_f64) * t76250 + t76253 - t77988;
    t77989
}
