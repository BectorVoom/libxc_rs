//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta293 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1504;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1505;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1506;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta293<F: Float>(t3087: F, t372: F, t364: F, t354: F, t1009: F, t3020: F, t1011: F, t1019: F, t1040: F, t3077: F, t2775: F, t283: F) -> (F, F, F, F, F, F, F) {
        let (t10956, t10957) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1504::<F>(t3087, t372, t364, t354);
        let (t10960, t10961, t10962) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1505::<F>(t1009, t3020, t1011, t1019);
        let t10965 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1506::<F>(t1040, t3077);
        let t10969 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1507::<F>(t2775, t283);
    (t10956, t10957, t10960, t10961, t10962, t10965, t10969)
}
