//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2324/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2324<F: Float>(t184: F, t20217: F, t4194: F, t607: F, t13126: F, t5398: F, t16558: F, t4195: F, t16620: F, t16693: F, t16689: F, t4202: F) -> (F, F, F, F, F) {
    let t67469 = t184 * t20217;
    let t67472 = F::new(12.0) * t4194 * t67469 * t607;
    let t67475 = F::new(36.0) * t4194 * t13126 * t5398;
    let t67478 = F::new(36.0) * t4194 * t4195 * t16558;
    let t67480 = F::new(36.0) * t16693 * t16620;
    let t67482 = F::new(12.0) * t16689 * t4202;
    (t67472, t67475, t67478, t67480, t67482)
}
