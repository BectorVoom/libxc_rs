//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2024;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta624<F: Float>(t23164: F, t23204: F, t25341: F, t1887: F, t81956: F, t25041: F, t215: F, t6581: F, t252: F, t81613: F, t23056: F, t25242: F, t6579: F, t25245: F, t82031: F, t25251: F, t23012: F, t7529: F, t23110: F, t23185: F, t25241: F, t1484: F, t852: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t87029, t87049, t87050, t87052, t87053, t87057, t87066) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2024::<F>(t23164, t23204, t25341, t1887, t81956, t25041, t215, t6581, t252, t81613, t23056, t25242, t6579);
        let (t87067, t87068, t87078, t87080, t87101, t87111) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2025::<F>(t87066, t25245, t82031, t25251, t87049, t23012, t7529, t23110, t23185, t25241, t1484, t852);
    (t87029, t87050, t87052, t87053, t87057, t87067, t87068, t87078, t87080, t87101, t87111)
}
