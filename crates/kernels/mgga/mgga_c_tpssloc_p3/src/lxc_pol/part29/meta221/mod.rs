//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1056;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta221<F: Float>(t1190: F, t1751: F, t1090: F, t1735: F, t3578: F, t1216: F, t1653: F, t1222: F, t1731: F, t1744: F, t1202: F, t1743: F, t225: F, t4940: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4947, t4949, t4950, t4953, t4954, t4957, t4959, t4961) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1056::<F>(t1190, t1751, t1090, t1735, t3578, t1216, t1653, t1222, t1731, t1744, t1202, t1743);
        let t4964 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1057::<F>(t225, t4940);
    (t4947, t4949, t4950, t4953, t4954, t4957, t4959, t4961, t4964)
}
