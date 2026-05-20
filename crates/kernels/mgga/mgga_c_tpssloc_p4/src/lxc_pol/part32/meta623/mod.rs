//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2030;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta623<F: Float>(t11820: F, t7339: F, t2122: F, t7319: F, t1235: F, t225: F, t461: F, t11553: F, t2121: F, t2123: F, t7288: F, t85660: F, t3427: F, t7295: F, t11947: F, t7394: F, t2157: F, t43706: F, t1453: F, t81439: F, t26129: F, t81442: F, t22470: F, t4067: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t86350, t86403, t86415, t86451, t86473) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2030::<F>(t11820, t7339, t2122, t7319, t1235, t225, t461, t11553, t2121, t2123, t7288, t85660);
        let (t86501, t86517, t86524, t86586, t86589, t86590) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2031::<F>(t2121, t3427, t7295, t11947, t7394, t2157, t43706, t1453, t81439, t26129, t81442, t22470, t4067);
    (t86350, t86403, t86415, t86451, t86473, t86501, t86517, t86524, t86586, t86589, t86590)
}
