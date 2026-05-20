//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta141 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk912;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk913;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk914;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk915;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk916;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk917;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk918;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta141<F: Float>(t3236: F, t3293: F, t3238: F, t3245: F, t3250: F, t3254: F, t3272: F, t3280: F, t3288: F, t3290: F, t3295: F, t3299: F, t3302: F, t3305: F, t1156: F, t1146: F, t440: F, t448: F, t3377: F, t1129: F, t1138: F, t1148: F, t1157: F, t3258: F, t3261: F, t3268: F, t3310: F, t3318: F, t3324: F, t3327: F, t3332: F, t3334: F, t3352: F, t3357: F, t3360: F, t3369: F, t3371: F, t3376: F, t3378: F, t436: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3383, t3390, t3395) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk912::<F>(t3236, t3293, t3238, t3245, t3250, t3254, t3272, t3280, t3288, t3290, t3295, t3299, t3302, t3305);
        let t3396 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk913::<F>(t1156, t3395);
        let t3399 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk914::<F>(t1146);
        let t3400 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk915::<F>(t3399);
        let t3401 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk916::<F>(t3400, t440);
        let (t3402, t3403) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk917::<F>(t448);
        let t3404 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk918::<F>(t3377, t3403);
        let t3407 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk919::<F>(t1129, t1138, t1148, t1157, t3258, t3261, t3268, t3310, t3318, t3324, t3327, t3332, t3334, t3352, t3357, t3360, t3369, t3371, t3376, t3378, t3396, t3401, t3404, t436);
    (t3383, t3390, t3395, t3396, t3399, t3400, t3401, t3402, t3403, t3404, t3407)
}
