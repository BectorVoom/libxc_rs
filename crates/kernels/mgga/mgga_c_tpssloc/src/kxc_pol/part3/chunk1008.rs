//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1008/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1008<F: Float>(t1527: F, t2719: F, t10110: F, t225: F, t4143: F, t2742: F, t2718: F, t4265: F, t798: F, t4145: F, t4142: F, t852: F) -> (F, F, F, F, F, F) {
    let t13049 = t1527 * t2719;
    let t13050 = t10110 * t13049;
    let t13053 = t4143 * t225;
    let t13058 = t1527 * t2742;
    let t13059 = t2718 * t13058;
    let t13062 = t798 * t4265;
    let t13065 = t4145 * t225;
    let t13068 = t4142 * t852;
    (t13050, t13053, t13059, t13062, t13065, t13068)
}
