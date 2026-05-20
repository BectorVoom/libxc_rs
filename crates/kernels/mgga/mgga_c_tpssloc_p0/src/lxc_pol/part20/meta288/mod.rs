//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1490;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1491;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1492;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1493;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta288<F: Float>(t10768: F, t10847: F, t300: F, t2940: F, t2944: F, t2924: F, t2929: F, t4497: F, t959: F, t10665: F, t10699: F, t10707: F, t10711: F, t10715: F, t10729: F, t10733: F, t10739: F, t10819: F, t10658: F, t360: F, t1021: F, t248: F, t1004: F, t3047: F, t3053: F, t3117: F, t1043: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10849, t10851, t10853, t10855, t10856) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1490::<F>(t10768, t10847, t300, t2940, t2944, t2924, t2929, t4497, t959, t10665, t10699, t10707, t10711, t10715, t10729, t10733, t10739, t10819);
        let t10857 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1491::<F>(t10658, t10856);
        let (t10858, t10860, t10863) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1492::<F>(t10857, t360, t1021, t248, t1004, t3047);
        let (t10866, t10868) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1493::<F>(t3053, t3117, t1043, t676);
    (t10849, t10851, t10853, t10855, t10857, t10858, t10860, t10863, t10866, t10868)
}
