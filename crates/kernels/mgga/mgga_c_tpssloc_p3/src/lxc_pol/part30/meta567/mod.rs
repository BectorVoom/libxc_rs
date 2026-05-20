//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta567<F: Float>(t1603: F, t7593: F, t5677: F, t6690: F, t23593: F, t23394: F, t5919: F, t6704: F, t5681: F, t6689: F, t1945: F, t5848: F) -> (F, F, F, F, F, F, F, F) {
        let (t28488, t28491, t28492, t28495, t28496, t28499, t28500, t28505) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1935::<F>(t1603, t7593, t5677, t6690, t23593, t23394, t5919, t6704, t5681, t6689, t1945, t5848);
    (t28488, t28491, t28492, t28495, t28496, t28499, t28500, t28505)
}
