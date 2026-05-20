//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta562<F: Float>(t28190: F, t28236: F, t533: F, t1390: F, t1983: F, t25: F, t5527: F, t1915: F, t1484: F, t1530: F) -> (F, F, F, F, F, F, F) {
        let (t28237, t28238, t28239, t28240, t28241, t28242, t28248) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1928::<F>(t28190, t28236, t533, t1390, t1983, t25, t5527, t1915, t1484, t1530);
    (t28237, t28238, t28239, t28240, t28241, t28242, t28248)
}
