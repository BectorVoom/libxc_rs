//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1667;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta354<F: Float>(t12300: F, t1354: F, t1307: F, t3719: F, t3870: F, t820: F, t12189: F, t1329: F, t3726: F, t3770: F, t119: F, t12012: F, t210: F, t12211: F, t3766: F) -> (F, F, F, F, F, F, F) {
        let (t12301, t12303) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1667::<F>(t12300, t1354, t1307, t3719);
        let (t12305, t12308, t12310, t12313, t12317) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1668::<F>(t12303, t3870, t820, t12189, t1329, t3726, t3770, t119, t12012, t210, t12211, t3766);
    (t12301, t12303, t12305, t12308, t12310, t12313, t12317)
}
