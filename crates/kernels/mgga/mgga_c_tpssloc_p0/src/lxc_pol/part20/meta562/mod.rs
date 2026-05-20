//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta562<F: Float>(t42340: F, t42341: F, t3034: F, t368: F, t3128: F, t10882: F, t42333: F, t1015: F, t1041: F, t10914: F, t13969: F, t10918: F) -> (F, F, F, F, F, F, F, F) {
        let (t42342, t42344, t42345, t42347, t42354, t42358, t42369, t42372) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2120::<F>(t42340, t42341, t3034, t368, t3128, t10882, t42333, t1015, t1041, t10914, t13969, t10918);
    (t42342, t42344, t42345, t42347, t42354, t42358, t42369, t42372)
}
