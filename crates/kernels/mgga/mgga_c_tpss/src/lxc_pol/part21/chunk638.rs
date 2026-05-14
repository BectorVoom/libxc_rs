//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 638/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk638<F: Float>(t2724: F, t2785: F, t948: F, t975: F, t2703: F, t366: F, t2712: F, t940: F, t2711: F) -> (F, F, F, F) {
    let t2786 = t2785 * t2724;
    let t2790 = t975 * t948;
    let t2794 = t366 * t2703;
    let t2797 = t2712 * t940;
    let t2798 = t2711 * t2797;
    (t2786, t2790, t2794, t2798)
}
