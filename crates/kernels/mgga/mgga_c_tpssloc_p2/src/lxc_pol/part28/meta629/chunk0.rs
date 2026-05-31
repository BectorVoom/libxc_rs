//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1970/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1970<F: Float>(t1877: F, t2057: F, t584: F, t9212: F, t2219: F, t7110: F, t26756: F, t86732: F, t86843: F, t86868: F, t86870: F, t225: F, t26722: F) -> (F, F, F, F, F, F, F, F) {
    let t92356 = t1877 * t2057 * t584;
    let t92359 = F::cast_from(3.0_f64) * t1877 * t2057 * t9212;
    let t92362 = F::cast_from(2.0_f64) * t1877 * t7110 * t2219;
    let t92364 = F::cast_from(2.0_f64) * t26756 * t86732;
    let t92375 = F::cast_from(0.76763589786250567036e-1_f64) * t86843;
    let t92382 = F::cast_from(0.15352717957250113407e0_f64) * t86868;
    let t92383 = F::cast_from(0.10417915756705434098e0_f64) * t86870;
    let t92386 = t26722 * t225;
    (t92356, t92359, t92362, t92364, t92375, t92382, t92383, t92386)
}
