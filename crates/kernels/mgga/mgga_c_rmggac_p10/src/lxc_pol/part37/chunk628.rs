//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 628/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk628<F: Float>(t118: F, t14519: F, t14521: F, t15007: F, t15146: F, t15557: F, t15559: F, t15560: F, t15561: F, t15562: F, t15870: F, t15872: F, t15885: F) -> F {
    let t15887 = t15870 + F::cast_from(0.31062809106223861414e-2_f64) * t15146 - t15557 + t14519 + t15559 - t15560 - t15561 - t14521 + t15007 + t15562 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t15872 + t15885;
    t15887
}
