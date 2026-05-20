//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta608<F: Float>(t10510: F, t6755: F, t11002: F, t23537: F, t10895: F, t23541: F, t23529: F, t3053: F, t10955: F, t1940: F, t354: F, t10459: F, t6765: F) -> (F, F, F, F, F, F) {
        let (t82851, t82859, t82861, t82863, t82868, t82871) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2081::<F>(t10510, t6755, t11002, t23537, t10895, t23541, t23529, t3053, t10955, t1940, t354, t10459, t6765);
    (t82851, t82859, t82861, t82863, t82868, t82871)
}
