//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta590<F: Float>(t39267: F, t404: F, t410: F, t407: F, t43819: F, t1098: F, t11470: F, t3256: F, t3312: F, t1094: F, t11274: F, t3262: F, t3311: F, t409: F) -> (F, F, F, F, F, F, F, F) {
        let (t43880, t43889, t43895, t43942, t43954, t43959, t43964, t43969) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2169::<F>(t39267, t404, t410, t407, t43819, t1098, t11470, t3256, t3312, t1094, t11274, t3262, t3311, t409);
    (t43880, t43889, t43895, t43942, t43954, t43959, t43964, t43969)
}
