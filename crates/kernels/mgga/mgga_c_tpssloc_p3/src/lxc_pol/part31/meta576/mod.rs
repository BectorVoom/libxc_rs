//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1812;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta576<F: Float>(t1864: F, t4021: F, t1410: F, t9231: F, t2240: F, t3961: F, t3967: F, t12571: F, t608: F, t645: F, t7445: F, t26351: F, t6883: F) -> (F, F, F, F, F, F, F) {
        let (t90094, t90098, t90101, t90104, t90114, t90247, t90459) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1812::<F>(t1864, t4021, t1410, t9231, t2240, t3961, t3967, t12571, t608, t645, t7445, t26351, t6883);
    (t90094, t90098, t90101, t90104, t90114, t90247, t90459)
}
