//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 966/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk966<F: Float>(t13126: F, t607: F, t4194: F, t12606: F, t185: F, t707: F, t4094: F, t706: F, t708: F, t9924: F, t9933: F, t13112: F, t13114: F, t13117: F, t13118: F, t13121: F, t13122: F, t13125: F, t9853: F, t9859: F, t9907: F, t9921: F) -> (F, F, F, F, F, F) {
    let t13127 = t13126 * t607;
    let t13129 = 24.0 * t4194 * t13127;
    let t13130 = t185 * t12606;
    let t13132 = 4.0 * t707 * t13130;
    let t13133 = t706 * t4094;
    let t13135 = 8.0 * t13133 * t708;
    let t13136 = 8.0 * t9924;
    let t13137 = 12.0 * t9933;
    let t13138 = -t13112 + t9907 - t13114 + t9853 + t13117 + t13118 - t13121 - t9921 - t13122 + t13125 + t13129 + t13132 + t13135 + t13136 + t9859 + t13137;
    (t13129, t13132, t13135, t13136, t13137, t13138)
}
