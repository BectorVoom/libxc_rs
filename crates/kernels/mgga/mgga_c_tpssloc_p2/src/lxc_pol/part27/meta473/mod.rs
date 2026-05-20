//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta473<F: Float>(t3034: F, t371: F, t1930: F, t6741: F, t3030: F, t3127: F, t363: F, t1011: F, t3040: F, t3131: F, t1014: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t23508, t23509, t23510, t23511, t23512, t23514, t23515, t23518, t23519, t23520) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1838::<F>(t3034, t371, t1930, t6741, t3030, t3127, t363, t1011, t3040, t3131, t1014, t360);
    (t23508, t23509, t23510, t23511, t23512, t23514, t23515, t23518, t23519, t23520)
}
