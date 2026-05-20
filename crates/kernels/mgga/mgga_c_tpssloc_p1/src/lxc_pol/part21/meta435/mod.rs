//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1972;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta435<F: Float>(t15402: F, t4729: F, t3447: F, t14736: F, t4900: F, t14740: F, t14731: F, t11575: F, t4904: F, t134: F, t3439: F, t461: F, t4724: F, t11514: F, t11556: F, t11558: F, t11561: F, t15391: F, t15396: F, t15401: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15403, t15405, t15406, t15409, t15412, t15415, t15418, t15419) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1972::<F>(t15402, t4729, t3447, t14736, t4900, t14740, t14731, t11575, t4904, t134, t3439, t461);
        let (t15420, t15422, t15423) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1973::<F>(t15419, t4724, t3447, t11514, t11556, t11558, t11561, t15391, t15396, t15401, t15405, t15406, t15409, t15412, t15415);
    (t15403, t15405, t15406, t15409, t15412, t15415, t15418, t15419, t15420, t15422, t15423)
}
