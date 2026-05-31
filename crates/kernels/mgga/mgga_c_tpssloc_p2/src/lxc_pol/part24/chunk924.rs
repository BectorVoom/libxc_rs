//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 924/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk924<F: Float>(t10309: F, t248: F, t3062: F, t3103: F, t3114: F, t376: F, t676: F, t1023: F, t1020: F, t1041: F, t10433: F, t10436: F, t10438: F, t10441: F, t10446: F, t10449: F, t10455: F, t10460: F, t10463: F, t10480: F, t10485: F, t10490: F, t10493: F, t10496: F, t3039: F, t3048: F, t3064: F, t3098: F, t3117: F, t3123: F, t378: F) -> (F, F, F) {
    let t10501 = t248 * t3062 * t10309;
    let t10504 = t3114 * t3103;
    let t10508 = t676 * t376;
    let t10510 = t248 * t10508 * t1023;
    let t10511 = t1020 * t10510;
    let t10513 = -t3039 * t10433 / F::cast_from(1024.0_f64) - t10436 / F::cast_from(4608.0_f64) + F::cast_from(19.0_f64) / F::cast_from(576.0_f64) * t10438 * t378 - t10441 / F::cast_from(144.0_f64) - F::cast_from(209.0_f64) / F::cast_from(2592.0_f64) * t10446 * t378 + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t10449 - F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t3048 * t3064 + t10455 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t10460 + t1041 * t10463 / F::cast_from(4608.0_f64) + t3114 * t3123 / F::cast_from(1024.0_f64) + t10480 * t10485 / F::cast_from(512.0_f64) - t10490 / F::cast_from(1152.0_f64) + t1041 * t10493 / F::cast_from(768.0_f64) - t10496 / F::cast_from(144.0_f64) - t3117 * t3098 / F::cast_from(768.0_f64) - F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t1041 * t10501 + t10504 / F::cast_from(768.0_f64) + t3048 * t3098 / F::cast_from(144.0_f64) - t10511 / F::cast_from(4608.0_f64);
    (t10501, t10510, t10513)
}
