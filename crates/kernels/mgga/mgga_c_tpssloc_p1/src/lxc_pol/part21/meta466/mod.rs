//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2039;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2040;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta466<F: Float>(t16148: F, t3870: F, t820: F, t1799: F, t3719: F, t3799: F, t5289: F, t11984: F, t15876: F, t15878: F, t15880: F, t15887: F, t15888: F, t15889: F, t15891: F, t15894: F, t15896: F, t15898: F, t15910: F, t9457: F, t9476: F, t9484: F, t9780: F, t12044: F, t12048: F, t12057: F, t12059: F, t12087: F, t12094: F, t15911: F, t15915: F, t15916: F, t15917: F, t15923: F, t15927: F, t15928: F, t9789: F, t9793: F, t9797: F, t12103: F, t12105: F, t12109: F, t12114: F, t12116: F, t12118: F, t12123: F, t15970: F, t15972: F, t15973: F, t15974: F, t15975: F, t15976: F, t15978: F, t9820: F, t9824: F) -> (F, F, F, F, F, F, F) {
        let (t16150, t16153, t16155, t16159, t16160) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2039::<F>(t16148, t3870, t820, t1799, t3719, t3799, t5289, t11984, t15876, t15878, t15880, t15887, t15888, t15889, t15891, t15894, t15896, t15898, t15910, t9457, t9476, t9484, t9780);
        let t16161 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2040::<F>(t12044, t12048, t12057, t12059, t12087, t12094, t15911, t15915, t15916, t15917, t15923, t15927, t15928, t9789, t9793, t9797);
        let t16163 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2041::<F>(t12103, t12105, t12109, t12114, t12116, t12118, t12123, t15970, t15972, t15973, t15974, t15975, t15976, t15978, t9820, t9824);
    (t16150, t16153, t16155, t16159, t16160, t16161, t16163)
}
