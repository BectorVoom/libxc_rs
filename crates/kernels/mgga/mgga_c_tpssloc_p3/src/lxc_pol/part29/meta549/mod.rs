//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1947;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta549<F: Float>(t27453: F, t27454: F, t1751: F, t477: F, t1090: F, t7362: F, t1653: F, t24858: F, t2144: F, t5011: F, t1246: F, t4733: F, t7363: F, t1215: F, t8054: F, t1244: F, t24760: F, t24773: F, t27406: F, t27451: F, t5064: F, t7283: F, t7365: F, t7387: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27455, t27460, t27461, t27462, t27465, t27466, t27470, t27471, t27473) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1947::<F>(t27453, t27454, t1751, t477, t1090, t7362, t1653, t24858, t2144, t5011, t1246, t4733, t7363);
        let (t27474, t27478, t27480) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1948::<F>(t27473, t7362, t1215, t8054, t1246, t1244, t24760, t24773, t27406, t27451, t27455, t27462, t27466, t27471, t5064, t7283, t7365, t7387);
    (t27455, t27460, t27461, t27462, t27465, t27466, t27470, t27471, t27473, t27474, t27478, t27480)
}
