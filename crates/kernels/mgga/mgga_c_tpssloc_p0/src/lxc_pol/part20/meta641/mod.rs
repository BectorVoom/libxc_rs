//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2348;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta641<F: Float>(t42444: F, t45971: F, t48140: F, t2770: F, t340: F, t43317: F, t136: F, t47746: F, t908: F, t2403: F, t4389: F, t4386: F, t13543: F, t699: F, t13547: F, t13556: F, t13529: F, t13533: F, t41887: F, t41889: F, t43002: F, t48122: F, t48125: F, t48128: F, t48131: F, t48134: F, t48137: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48142, t48145, t48148, t48153, t48155, t48156, t48157) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2348::<F>(t42444, t45971, t48140, t2770, t340, t43317, t136, t47746, t908, t2403, t4389, t4386);
        let (t48159, t48161, t48163, t48165, t48167, t48169) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2349::<F>(t48157, t13543, t699, t13547, t13556, t13529, t13533, t41887, t41889, t43002, t48122, t48125, t48128, t48131, t48134, t48137, t48142, t48145, t48148, t48153, t48156);
    (t48142, t48145, t48148, t48153, t48155, t48157, t48159, t48161, t48163, t48165, t48167, t48169)
}
