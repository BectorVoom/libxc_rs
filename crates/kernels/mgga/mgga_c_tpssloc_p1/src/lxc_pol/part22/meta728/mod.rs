//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta728 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2382;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2383;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2384;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2385;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2386;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2387;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2388;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta728<F: Float>(t136: F, t68554: F, t908: F, t43317: F, t48140: F, t68513: F, t49200: F, t59657: F, t60163: F, t60168: F, t60173: F, t68536: F, t68541: F, t68545: F, t68549: F, t68552: F, t67060: F, t883: F, t123: F, t882: F, t68543: F, t68547: F, t68458: F, t68534: F, t2768: F, t68539: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t68556, t68563, t68565) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2382::<F>(t136, t68554, t908, t43317, t48140, t68513, t49200, t59657, t60163, t60168, t60173, t68536, t68541, t68545, t68549, t68552);
        let (t68569, t68571) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2383::<F>(t67060, t883, t123, t882);
        let t68577 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2384::<F>(t123, t68543, t882);
        let t68580 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2385::<F>(t123, t68547, t882);
        let t68583 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2386::<F>(t123, t68554, t882);
        let t68586 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2387::<F>(t123, t68458, t882);
        let t68589 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2388::<F>(t123, t68534, t882);
        let t68592 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2389::<F>(t123, t2768, t68539);
    (t68556, t68563, t68565, t68569, t68571, t68577, t68580, t68583, t68586, t68589, t68592)
}
