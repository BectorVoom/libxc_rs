//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta686 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta686<F: Float>(t3400: F, t6063: F, t1098: F, t18245: F, t3312: F, t5983: F, t18496: F, t699: F, t18517: F, t18514: F, t18520: F, t2403: F, t6011: F) -> (F, F, F, F, F, F, F, F) {
        let (t63602, t63750, t63755, t63841, t63843, t63845, t63886, t63888) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2263::<F>(t3400, t6063, t1098, t18245, t3312, t5983, t18496, t699, t18517, t18514, t18520, t2403, t6011);
    (t63602, t63750, t63755, t63841, t63843, t63845, t63886, t63888)
}
