//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta190 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk913;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk914;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta190<F: Float>(t475: F, t5011: F, t1214: F, t248: F, t1017: F, t1742: F, t1210: F, t1207: F, t372: F, t479: F, t471: F, t1230: F, t4733: F, t3440: F, t4724: F, t1193: F, t1706: F, t135: F, t1725: F, t1174: F, t1196: F, t3966: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5012, t5014, t5018, t5019) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk913::<F>(t475, t5011, t1214, t248, t1017, t1742, t1210, t1207);
        let (t5023, t5024) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk914::<F>(t1742, t372, t479, t471);
        let (t5030, t5033, t5036, t5040, t5041, t5045) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk915::<F>(t1230, t248, t4733, t3440, t4724, t1193, t1706, t135, t1725, t1174, t1196, t3966);
    (t5012, t5014, t5018, t5019, t5023, t5024, t5030, t5033, t5036, t5040, t5041, t5045)
}
