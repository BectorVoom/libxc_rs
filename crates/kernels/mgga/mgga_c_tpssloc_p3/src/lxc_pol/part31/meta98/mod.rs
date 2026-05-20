//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk603;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk604;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta98<F: Float>(t84: F, t85: F, t24: F, t42: F, t54: F, t240: F, t59: F, t40: F, t632: F, t73: F, t52: F, t636: F, t76: F, t111: F, t649: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t2239 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk603::<F>(t84, t85);
        let t2240 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk604::<F>(t2239, t24);
        let (t2267, t2274, t2281, t2282, t2289, t2291, t2296, t2298, t2314) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk605::<F>(t42, t54, t240, t59, t40, t632, t73, t52, t636, t76, t111, t649);
    (t2239, t2240, t2267, t2274, t2281, t2282, t2289, t2291, t2296, t2298, t2314)
}
