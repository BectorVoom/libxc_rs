//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1106/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1106<F: Float>(t3281: F, t551: F, t321: F, t333: F, t4669: F, t5148: F, t5266: F, t76246: F, t76250: F, t76253: F, t77988: F, t77992: F, t77995: F, t77996: F, t77997: F, t77998: F, t78005: F, t78006: F, t80444: F) -> (F, F) {
    let t80452 = t3281 * t551;
    let t80462 = F::cast_from(0.15531404553111930707e-1_f64) * t76246 + F::cast_from(0.31062809106223861414e-2_f64) * t76250 + t76253 - F::cast_from(0.17961362552795712846e0_f64) * t4669 * t80452 * t333 - t77988 - t77992 - t77995 - F::cast_from(0.11974241701863808564e0_f64) * t5148 * t80444 * t321 + F::cast_from(0.11974241701863808564e0_f64) * t5266 * t80444 * t333 - t77996 - t77997 - t77998 + t78005 - t78006;
    (t80452, t80462)
}
