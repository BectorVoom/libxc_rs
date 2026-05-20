//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta706 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2206;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta706<F: Float>(t25010: F, t7685: F, t16944: F, t25014: F, t25365: F, t86721: F, t22960: F, t67128: F, t1877: F, t2219: F, t7541: F, t5527: F, t606: F, t1915: F, t22959: F, t25013: F, t25024: F, t2522: F, t25354: F, t25358: F, t25377: F, t25392: F, t28241: F, t28242: F, t28252: F, t28256: F, t28456: F, t4314: F, t46341: F, t6666: F, t7475: F, t81539: F) -> (F, F, F) {
        let (t97949, t97950, t97953, t97956, t97972, t97985) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2206::<F>(t25010, t7685, t16944, t25014, t25365, t86721, t22960, t67128, t1877, t2219, t7541, t5527, t606);
        let t97989 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2207::<F>(t1877, t1915, t22959, t25013, t25024, t2522, t25354, t25358, t25377, t25392, t28241, t28242, t28252, t28256, t28456, t4314, t46341, t6666, t7475, t7541, t81539, t97950, t97953, t97956, t97972, t97985);
    (t97949, t97972, t97989)
}
