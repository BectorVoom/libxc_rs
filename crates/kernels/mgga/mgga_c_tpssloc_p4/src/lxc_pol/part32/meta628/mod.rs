//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2038;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta628<F: Float>(t23012: F, t7529: F, t23110: F, t23185: F, t25241: F, t1484: F, t852: F, t252: F, t4119: F, t25160: F, t814: F, t22690: F, t7520: F, t81573: F, t25324: F, t6562: F, t794: F, t23030: F, t25258: F, t22893: F, t23164: F, t25306: F, t7524: F, t81612: F, t81613: F, t4250: F, t81749: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t87080, t87101, t87111, t87130, t87135, t87140) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2038::<F>(t23012, t7529, t23110, t23185, t25241, t1484, t852, t252, t4119, t25160, t814, t22690, t7520, t81573);
        let (t87154, t87155, t87166, t87177, t87197) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2039::<F>(t25324, t6562, t794, t23030, t25258, t22893, t23164, t25306, t7524, t81612, t81613, t4250, t81749);
    (t87080, t87101, t87111, t87130, t87135, t87140, t87154, t87155, t87166, t87177, t87197)
}
