//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1712;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta335<F: Float>(t25: F, t28: F, t4021: F, t645: F, t1437: F, t2307: F, t1409: F, t9321: F, t2291: F, t3966: F, t584: F, t9212: F, zeta_threshold: F, t9330: F, t2298: F, t2244: F, t2250: F, t4007: F, t4012: F, t607: F, t634: F, t638: F) -> (F, F, F, F, F, F, F, F) {
        let (t12585, t12588, t12595, t12598, t12603, t12604, t12606) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1712::<F>(t25, t28, t4021, t645, t1437, t2307, t1409, t9321, t2291, t3966, t584, t9212, zeta_threshold);
        let (t12609, t12619) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1713::<F>(t1409, t9330, t2298, t3966, t12595, t12598, t12606, t2244, t2250, t4007, t4012, t607, t634, t638);
    (t12585, t12588, t12595, t12603, t12604, t12606, t12609, t12619)
}
