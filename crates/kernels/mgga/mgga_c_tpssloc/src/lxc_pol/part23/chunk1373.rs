//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1373/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1373<F: Float>(t119: F, t16311: F, t19876: F, t20475: F, t210: F, t3733: F, t40025: F, t5246: F, t5248: F, t54151: F, t56927: F, t56946: F, t56953: F, t56993: F, t57011: F, t57019: F, t57041: F, t57073: F, t74090: F, t79921: F, t80021: F) -> (F,) {
    let t80399 = 595.0 / 2592.0 * t54151 - 119.0 / 2304.0 * t56927 + 5.0 / 4.0 * t40025 * t210 * t119 * t80021 + 3.0 / 16.0 * t3733 * t210 * t119 * t79921 + 35.0 / 12.0 * t56946 - 35.0 / 36.0 * t56953 + 119.0 / 288.0 * t56993 + 595.0 / 576.0 * t57011 - 119.0 / 576.0 * t57019 + 119.0 / 1152.0 * t57041 - 119.0 / 1152.0 * t57073 + t5246 * t5248 * t74090 * t16311 / 384.0 + t19876 * t20475 / 128.0;
    (t80399,)
}
