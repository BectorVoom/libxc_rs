//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta202 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta202<F: Float>(t491: F, t4940: F, t1235: F, t1720: F, t1721: F, t225: F, t1190: F, t1751: F, t1090: F, t1735: F, t3578: F, t1216: F, t1653: F) -> (F, F, F, F, F, F, F) {
        let (t4941, t4943, t4945, t4947, t4949, t4950, t4953) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1211::<F>(t491, t4940, t1235, t1720, t1721, t225, t1190, t1751, t1090, t1735, t3578, t1216, t1653);
    (t4941, t4943, t4945, t4947, t4949, t4950, t4953)
}
