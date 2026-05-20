//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 809/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk809<F: Float>(t1023: F, t10508: F, t248: F, t1020: F, t1041: F, t10433: F, t10436: F, t10438: F, t10441: F, t10446: F, t10449: F, t10455: F, t10460: F, t10463: F, t10480: F, t10485: F, t10490: F, t10493: F, t10496: F, t10501: F, t10504: F, t3039: F, t3048: F, t3064: F, t3098: F, t3114: F, t3117: F, t3123: F, t378: F) -> F {
    let t10510 = t248 * t10508 * t1023;
    let t10511 = t1020 * t10510;
    let t10513 = -t3039 * t10433 / F::new(1024.0) - t10436 / F::new(4608.0) + F::new(19.0) / F::new(576.0) * t10438 * t378 - t10441 / F::new(144.0) - F::new(209.0) / F::new(2592.0) * t10446 * t378 + F::new(19.0) / F::new(864.0) * t10449 - F::new(5.0) / F::new(864.0) * t3048 * t3064 + t10455 / F::new(2304.0) + F::new(5.0) / F::new(6912.0) * t10460 + t1041 * t10463 / F::new(4608.0) + t3114 * t3123 / F::new(1024.0) + t10480 * t10485 / F::new(512.0) - t10490 / F::new(1152.0) + t1041 * t10493 / F::new(768.0) - t10496 / F::new(144.0) - t3117 * t3098 / F::new(768.0) - F::new(5.0) / F::new(2304.0) * t1041 * t10501 + t10504 / F::new(768.0) + t3048 * t3098 / F::new(144.0) - t10511 / F::new(4608.0);
    t10513
}
