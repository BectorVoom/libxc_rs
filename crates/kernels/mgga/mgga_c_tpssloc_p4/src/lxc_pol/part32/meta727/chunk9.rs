//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2362/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2362<F: Float>(t100828: F, t100833: F, t100835: F, t100838: F, t100840: F, t100854: F, t100861: F, t100863: F, t1266: F, t29493: F, t4026: F, t5107: F, t7983: F, t8103: F, t97930: F, t97932: F, t97935: F, t97937: F, t97941: F, t97942: F, t97947: F, t97949: F) -> F {
    let t105099 = -F::new(2.0) * t1266 * t29493 - F::new(2.0) * t4026 * t8103 - F::new(2.0) * t5107 * t7983 + t100828 - t100833 - t100835 + t100838 - t100840 + t100854 + t100861 - t100863 + t97930 - t97932 - t97935 - t97937 + t97941 + t97942 - t97947 - t97949;
    t105099
}
