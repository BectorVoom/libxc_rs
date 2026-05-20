//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1269;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1270;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1271;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1272;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1273;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta281<F: Float>(t1751: F, t225: F, t497: F, t462: F, t1760: F, t7301: F, t7300: F, t1720: F, t2144: F, t131: F, t7998: F, t2130: F, t1932: F, rho1: F, t2133: F, t2132: F, t7573: F, t1714: F, t460: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t8009, t8010) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1269::<F>(t1751, t225, t497);
        let (t8011, t8014, t8015) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1270::<F>(t462, t8010, t1760, t7301, t7300);
        let (t8018, t8020) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1271::<F>(t1720, t2144, t131, t7998);
        let (t8026, t8027) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1272::<F>(t2130, t1932, rho1);
        let (t8028, t8031, t8034) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1273::<F>(t2133, t8027, t2132, t7573, t1714, t460);
    (t8009, t8010, t8011, t8014, t8015, t8018, t8020, t8026, t8027, t8028, t8031, t8034)
}
