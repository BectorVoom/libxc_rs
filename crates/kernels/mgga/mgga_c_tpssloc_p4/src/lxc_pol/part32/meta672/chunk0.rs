//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2105/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2105<F: Float>(t15643: F, t7345: F, t27639: F, t86264: F, t27645: F, t3540: F, t8043: F, t2136: F, t607: F, t8027: F, t1714: F, t24682: F, t460: F) -> (F, F, F, F, F, F, F) {
    let t95352 = t7345 * t15643 / F::cast_from(864.0_f64);
    let t95362 = F::cast_from(0.40372756094140390856e-3_f64) * t86264 * t27639;
    let t95364 = F::cast_from(0.20186378047070195428e-3_f64) * t86264 * t27645;
    let t95365 = t8043 * t3540;
    let t95370 = F::cast_from(0.16149102437656156342e-2_f64) * t8027 * t607 * t2136;
    let t95382 = t607 * t1714;
    let t95384 = t24682 * t95382 * t460;
    (t95352, t95362, t95364, t95365, t95370, t95382, t95384)
}
