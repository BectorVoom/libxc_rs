//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1123/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1123<F: Float>(t12410: F, t3068: F, t1562: F, t2841: F, t9702: F, t1111: F, t1125: F, t12330: F, t12355: F, t12361: F, t12363: F, t12368: F, t12371: F, t12374: F, t12381: F, t12385: F, t12391: F, t12395: F, t12401: F, t12406: F, t12409: F, t3067: F, t9556: F, t9563: F, t9573: F, t9633: F, t9658: F, t9661: F) -> F {
    let t12411 = t3068 * t12410;
    let t12414 = t1562 * t2841;
    let t12415 = t9702 * t12414;
    let t12421 = -t9556 * t12330 / F::cast_from(2304.0_f64) + t1111 * t12355 / F::cast_from(3072.0_f64) - t12361 - t1125 * t12363 / F::cast_from(4608.0_f64) + t12368 / F::cast_from(20736.0_f64) - t12371 - t3067 * t12374 / F::cast_from(1152.0_f64) + t9573 * t12381 / F::cast_from(2304.0_f64) + t12385 / F::cast_from(1296.0_f64) - t9556 * t12391 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t3067 * t12395 - t9563 / F::cast_from(3456.0_f64) - t1125 * t12401 / F::cast_from(768.0_f64) + t12406 + t12409 + t9573 * t12411 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t3067 * t12415 + t9633 / F::cast_from(648.0_f64) + t9658 / F::cast_from(648.0_f64) - t9661 / F::cast_from(864.0_f64);
    t12421
}
