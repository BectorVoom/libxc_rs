//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta501<F: Float>(t154: F, t2558: F, t10: F, t2229: F, t116: F, t117: F, t556: F, t243: F, t3008: F, t343: F, t3034: F, t371: F) -> (F, F, F, F, F, F, F) {
        let (t22715, t22811, t22815, t22843, t23076, t23494, t23508) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2010::<F>(t154, t2558, t10, t2229, t116, t117, t556, t243, t3008, t343, t3034, t371);
    (t22715, t22811, t22815, t22843, t23076, t23494, t23508)
}
