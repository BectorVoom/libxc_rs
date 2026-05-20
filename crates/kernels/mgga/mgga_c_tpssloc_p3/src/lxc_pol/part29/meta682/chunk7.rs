//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2311/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2311<F: Float>(t24668: F, t27497: F, t15643: F, t7345: F, t27639: F, t86264: F, t27645: F, t3540: F, t8043: F, t15545: F, t15667: F, t24699: F, t24749: F, t27655: F, t7310: F, t7316: F, t8028: F, t8035: F, t86191: F, t86234: F) -> F {
    let t95346 = t24668 * t27497;
    let t95352 = t7345 * t15643 / F::new(864.0);
    let t95362 = F::cast_from(0.40372756094140390856e-3_f64) * t86264 * t27639;
    let t95364 = F::cast_from(0.20186378047070195428e-3_f64) * t86264 * t27645;
    let t95365 = t8043 * t3540;
    let t95367 = F::cast_from(0.20186378047070195428e-3_f64) * t86234 * t95346 + F::new(5.0) / F::new(6912.0) * t7345 * t15545 - t95352 + F::cast_from(0.20186378047070195428e-3_f64) * t7316 * t27655 + F::cast_from(0.10093189023535097714e-3_f64) * t24749 * t8035 - t7310 * t15667 / F::new(288.0) + F::cast_from(0.80745512188280781712e-3_f64) * t8028 * t24699 + t95362 - t95364 - t95365 / F::new(6912.0) + t86191;
    t95367
}
