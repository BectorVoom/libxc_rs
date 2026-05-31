//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2331/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2331<F: Float>(t28525: F, t461: F, t7324: F, t18342: F, t18346: F, t18590: F, t18594: F, t18997: F, t19068: F, t27604: F, t27617: F, t27674: F, t4974: F, t4989: F, t5046: F, t7310: F, t7331: F, t7345: F, t95550: F, t95571: F, t95573: F) -> F {
    let t104387 = t7324 * t28525 * t461;
    let t104404 = t95550 / F::cast_from(5184.0_f64) + t27674 * t5046 / F::cast_from(54.0_f64) - t7310 * t18997 / F::cast_from(288.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t104387 * t7331 + t95571 - t95573 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t7345 * t18342 + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t7345 * t18346 - t7345 * t18590 / F::cast_from(576.0_f64) - t7345 * t18594 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t7345 * t19068 - F::cast_from(5.0_f64) / F::cast_from(648.0_f64) * t27604 * t4989 - t27617 * t4974 / F::cast_from(576.0_f64);
    t104404
}
