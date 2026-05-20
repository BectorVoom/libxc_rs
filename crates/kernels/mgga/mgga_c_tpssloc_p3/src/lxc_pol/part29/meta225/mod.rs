//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta225<F: Float>(t475: F, t5011: F, t1214: F, t248: F, t1017: F, t1742: F, t1210: F, t1207: F, t372: F, t479: F, t471: F) -> (F, F, F, F, F, F, F, F) {
        let (t5012, t5014, t5017, t5018, t5019, t5022, t5023, t5024) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1062::<F>(t475, t5011, t1214, t248, t1017, t1742, t1210, t1207, t372, t479, t471);
    (t5012, t5014, t5017, t5018, t5019, t5022, t5023, t5024)
}
