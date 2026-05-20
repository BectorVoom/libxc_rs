//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1070;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1071;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta220<F: Float>(t3578: F, t4953: F, t1222: F, t1731: F, t1744: F, t1202: F, t1743: F, t225: F, t4940: F, t68: F, t484: F, t1177: F, t4729: F, t1229: F, t3247: F, t3961: F, t4582: F, t1734: F, t486: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4954, t4957, t4959, t4961, t4964, t4965, t4966, t4969) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1070::<F>(t3578, t4953, t1222, t1731, t1744, t1202, t1743, t225, t4940, t68, t484, t1177, t4729);
        let (t4972, t4973, t4974, t4977) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1071::<F>(t1229, t3247, t3961, t4582, t1734, t486);
    (t4954, t4957, t4959, t4961, t4964, t4965, t4966, t4969, t4972, t4973, t4974, t4977)
}
