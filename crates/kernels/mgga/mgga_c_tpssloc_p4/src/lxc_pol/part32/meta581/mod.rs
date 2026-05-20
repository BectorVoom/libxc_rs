//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1963;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1964;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta581<F: Float>(t2123: F, t6146: F, t2144: F, t6150: F, t1720: F, t8054: F, t5971: F, t7286: F, t24595: F, t27426: F, t8002: F, t2121: F, t2124: F, t27755: F, t27770: F, t29671: F, t29674: F, t29678: F, t498: F, t7283: F, t7999: F, t8011: F, t2148: F, t6140: F, t6224: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t29682, t29685, t29687, t29690, t29691, t29694, t29699) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1963::<F>(t2123, t6146, t2144, t6150, t1720, t8054, t5971, t7286, t24595, t27426, t8002, t2121, t2124, t27755, t27770, t29671, t29674, t29678, t498, t7283, t7999, t8011);
        let (t29702, t29705, t29708) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1964::<F>(t2148, t6146, t6140, t2144, t6224);
    (t29682, t29685, t29687, t29690, t29691, t29694, t29699, t29702, t29705, t29708)
}
