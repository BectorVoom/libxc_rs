//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 604/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk604<F: Float>(t2724: F, t2785: F, t948: F, t975: F, t2703: F, t366: F, t2712: F, t940: F, t2711: F, t345: F, t220: F, t2768: F, t2782: F, t2783: F, t368: F, t983: F, t985: F) -> (F, F, F, F) {
    let t2786 = t2785 * t2724;
    let t2790 = t975 * t948;
    let t2794 = t366 * t2703;
    let t2797 = t2712 * t940;
    let t2798 = t2711 * t2797;
    let t2799 = t2785 * t345;
    let t2804 = t220 * t2768 * t368 + 2.0 * t2782 * t2783 * t2786 - t2783 * t2798 * t2799 + 2.0 * t2790 * t983 * t985 + t2794 * t983 * t985;
    (t2786, t2798, t2799, t2804)
}
