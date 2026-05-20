//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2419/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2419<F: Float>(t291: F, t68972: F, t68992: F, t21100: F, t4497: F, t959: F, t68934: F, t68936: F, t68938: F, t68940: F, t68943: F, t68947: F, t68949: F, t68951: F, t68954: F) -> (F, F, F) {
    let t68995 = F::new(0.621814e-1) * (t68972 + t68992) * t291;
    let t68998 = F::cast_from(0.6233709278045326953e3_f64) * t959 * t21100 * t4497;
    let t68999 = -t68934 - t68936 - t68938 + t68940 + t68943 + t68947 + t68949 - t68951 - t68954 - t68995 - t68998;
    (t68995, t68998, t68999)
}
