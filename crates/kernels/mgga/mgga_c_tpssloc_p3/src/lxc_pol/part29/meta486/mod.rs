//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta486<F: Float>(t24702: F, t24756: F, t466: F, t24574: F, t7368: F, t2148: F, t3477: F, t1186: F, t7381: F, t3427: F, t2121: F, t225: F, t24594: F) -> (F, F, F, F, F, F, F, F) {
        let (t24757, t24758, t24760, t24762, t24765, t24771, t24773, t24776) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1831::<F>(t24702, t24756, t466, t24574, t7368, t2148, t3477, t1186, t7381, t3427, t2121, t225, t24594);
    (t24757, t24758, t24760, t24762, t24765, t24771, t24773, t24776)
}
