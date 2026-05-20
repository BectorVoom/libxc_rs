//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta710 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2306;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta710<F: Float>(t57965: F, t40722: F, t40733: F, t57992: F, t185: F, t67060: F, t707: F, t21066: F, t2752: F, t145: F, t67083: F, t20767: F, t751: F, t16596: F, t16662: F, t17116: F, t1877: F, t2522: F, t39483: F, t40732: F, t4310: F, t46237: F, t868: F) -> (F, F, F, F, F, F, F, F) {
        let (t67137, t67141, t67146, t67147, t67153, t67154, t67158, t67159) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2306::<F>(t57965, t40722, t40733, t57992, t185, t67060, t707, t21066, t2752, t145, t67083, t20767, t751);
        let t67160 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2307::<F>(t16596, t16662, t17116, t1877, t2522, t39483, t40732, t4310, t46237, t67146, t67147, t67153, t67154, t67158, t67159, t868);
    (t67137, t67141, t67146, t67147, t67153, t67158, t67159, t67160)
}
