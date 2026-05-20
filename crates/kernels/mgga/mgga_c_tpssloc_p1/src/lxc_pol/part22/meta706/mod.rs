//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta706 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta706<F: Float>(t1243: F, t65955: F, t19253: F, t225: F, t19121: F, t19259: F, t11947: F, t6270: F, t112: F, t20148: F, t1851: F, t5381: F) -> (F, F, F, F, F, F, F) {
        let (t66787, t66822, t66845, t66860, t66897, t66958, t66964) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2296::<F>(t1243, t65955, t19253, t225, t19121, t19259, t11947, t6270, t112, t20148, t1851, t5381);
    (t66787, t66822, t66845, t66860, t66897, t66958, t66964)
}
