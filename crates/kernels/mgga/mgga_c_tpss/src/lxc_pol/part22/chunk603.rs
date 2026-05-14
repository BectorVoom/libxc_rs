//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 603/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk603<F: Float>(t2688: F, t2767: F, t219: F, t976: F, t371: F, t979: F, t73: F, t990: F, t2711: F, t2712: F, t2715: F, t2723: F, t366: F, t329: F, t356: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2768 = t2688 + t2767;
    let t2769 = param_beta * t2768;
    let t2771 = t976 * t219;
    let t2775 = 1.0 / t979 / t371;
    let t2776 = t73 * t2775;
    let t2777 = t990 * t990;
    let t2778 = t2776 * t2777;
    let t2782 = t2711 * t2712 * t2715;
    let t2783 = t366 * t2723;
    let t2785 = 1.0 / t356 / t329;
    (t2768, t2769, t2771, t2776, t2777, t2778, t2782, t2783, t2785)
}
