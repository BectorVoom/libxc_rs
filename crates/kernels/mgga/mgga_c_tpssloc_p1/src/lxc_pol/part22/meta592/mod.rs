//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta592<F: Float>(t46953: F, t41466: F, t820: F, t13176: F, t2642: F, t10024: F, t1500: F, t41115: F, t4191: F, t4166: F, t9670: F, t831: F) -> (F, F, F, F, F, F, F) {
        let (t46954, t47039, t47044, t47047, t47080, t47092, t47093) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2108::<F>(t46953, t41466, t820, t13176, t2642, t10024, t1500, t41115, t4191, t4166, t9670, t831);
    (t46954, t47039, t47044, t47047, t47080, t47092, t47093)
}
