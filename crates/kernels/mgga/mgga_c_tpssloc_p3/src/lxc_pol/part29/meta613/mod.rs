//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2053;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta613<F: Float>(t24591: F, t85639: F, t24698: F, t491: F, t2127: F, t82631: F, t7291: F, t24564: F, t24574: F, t11605: F, t225: F, t3597: F, t3599: F, t2122: F, t7303: F, t3590: F, t7299: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t85640, t85648, t85660) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2053::<F>(t24591, t85639, t24698, t491, t2127, t82631);
        let (t85661, t85669, t85674, t85688, t85701, t85707) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2054::<F>(t7291, t85660, t24564, t24574, t11605, t225, t3597, t3599, t2122, t7303, t3590, t7299);
    (t85640, t85648, t85660, t85661, t85669, t85674, t85688, t85701, t85707)
}
