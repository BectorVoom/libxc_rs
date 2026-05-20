//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2230/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2230<F: Float>(t10190: F, t17817: F, t2986: F, t17769: F, t2960: F, t10224: F, t5824: F, t973: F, t13822: F, t17752: F, t17757: F, t17772: F, t2970: F) -> (F, F, F, F, F, F) {
    let t61397 = t2986 * t10190 * t17817;
    let t61405 = t2960 * t17769;
    let t61408 = t973 * t10224 * t5824;
    let t61422 = t973 * t13822 * t17752;
    let t61427 = t973 * t13822 * t17757;
    let t61447 = t973 * t2970 * t17772;
    (t61397, t61405, t61408, t61422, t61427, t61447)
}
