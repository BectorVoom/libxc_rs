//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta658 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta658<F: Float>(t407: F, t43819: F, t3256: F, t3312: F, t1094: F, t11274: F, t3262: F, t3311: F, t409: F, t11285: F, t3395: F, t43776: F) -> (F, F, F, F, F, F, F, F) {
        let (t43889, t43895, t43942, t43959, t43964, t43969, t43984, t44027) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2459::<F>(t407, t43819, t3256, t3312, t1094, t11274, t3262, t3311, t409, t11285, t3395, t43776);
    (t43889, t43895, t43942, t43959, t43964, t43969, t43984, t44027)
}
