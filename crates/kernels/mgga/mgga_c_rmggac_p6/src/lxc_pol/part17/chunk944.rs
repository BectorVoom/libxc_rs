//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 944/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk944<F: Float>(t10066: F, t36343: F, t236: F, t498: F, t7230: F, t9210: F, t9988: F, t321: F, t7248: F, t1810: F, t7754: F, t2010: F, t7756: F) -> (F, F, F, F) {
    let t45688 = t36343 * t10066;
    let t45696 = t7230 * t9210 * t236 * t9988 * t498;
    let t45701 = t7230 * t7248 * t236 * t9988 * t321;
    let t45707 = t7754 * t1810;
    let t45709 = t2010 * t45707 * t7756;
    (t45688, t45696, t45701, t45709)
}
