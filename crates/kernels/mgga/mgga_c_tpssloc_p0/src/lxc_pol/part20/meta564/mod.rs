//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta564<F: Float>(t10413: F, t10414: F, t10422: F, t10393: F, t3070: F, t11046: F, t42387: F, t10457: F, t820: F, t10409: F, t10936: F, t3180: F) -> (F, F, F, F, F, F) {
        let (t42478, t42481, t42483, t42488, t42490, t42496) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2123::<F>(t10413, t10414, t10422, t10393, t3070, t11046, t42387, t10457, t820, t10409, t10936, t3180);
    (t42478, t42481, t42483, t42488, t42490, t42496)
}
