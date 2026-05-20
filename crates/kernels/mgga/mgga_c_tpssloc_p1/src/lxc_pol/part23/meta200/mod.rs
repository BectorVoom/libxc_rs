//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta200<F: Float>(t11717: F, t3503: F, t11713: F, t1210: F, t11153: F, t3439: F, t11147: F, t11545: F, t3247: F, t415: F, t61: F, t121: F, t3584: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11727, t11728, t11737, t11738, t11759, t11764, t11778, t11779, t11784) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk841::<F>(t11717, t3503, t11713, t1210, t11153, t3439, t11147, t11545, t3247, t415, t61, t121, t3584);
    (t11727, t11728, t11737, t11738, t11759, t11764, t11778, t11779, t11784)
}
