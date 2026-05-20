//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta551<F: Float>(t27496: F, t27497: F, t5083: F, t7376: F, t7375: F, t1419: F, t6794: F, t131: F, t467: F, t5075: F, t225: F, t8034: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t27498, t27501, t27502, t27505, t27506, t27507, t27510, t27511, t27516) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1950::<F>(t27496, t27497, t5083, t7376, t7375, t1419, t6794, t131, t467, t5075, t225, t8034);
    (t27498, t27501, t27502, t27505, t27506, t27507, t27510, t27511, t27516)
}
