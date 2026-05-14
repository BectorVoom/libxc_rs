//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 878/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk878<F: Float>(t10027: F, t222: F, t2606: F, t9573: F, t805: F, t9541: F, t2563: F, t2610: F, t225: F, t2592: F, t2710: F, t814: F, t856: F, t68: F, t2745: F, t870: F) -> (F, F, F, F, F, F, F, F) {
    let t10029 = 455.0 / 1296.0 * t10027 * t222;
    let t10030 = t9573 * t2606;
    let t10036 = t9541 * t805;
    let t10038 = t2563 * t2610;
    let t10049 = t2592 * t225;
    let t10076 = t814 * t2710;
    let t10108 = t856 * t856;
    let t10109 = 1.0 / t10108;
    let t10110 = t68 * t10109;
    let t10126 = t2745 * t870;
    (t10029, t10030, t10036, t10038, t10049, t10076, t10110, t10126)
}
