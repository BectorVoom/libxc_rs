//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1217/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1217<F: Float>(t19755: F, t20021: F, t1378: F, t1385: F, t6460: F, t3887: F, t225: F, t6364: F, t20009: F, t539: F, t1375: F, t1386: F, t16030: F, t16439: F, t1843: F, t19635: F, t19644: F, t19648: F, t3882: F, t5321: F, t5326: F, t5354: F, t568: F, t6461: F) -> F {
    let t20022 = t19755 + t20021;
    let t20023 = t1378 * t20022;
    let t20025 = t6460 * t1385;
    let t20026 = t3887 * t20025;
    let t20029 = t6364 * t225;
    let t20032 = t539 * t20009;
    let t20034 = F::new(4.0) * t1375 * t19648 - t1375 * t20023 + F::new(2.0) * t1375 * t20026 - F::new(2.0) * t1386 * t20029 - F::new(2.0) * t16030 * t1843 - F::new(2.0) * t16439 * t1843 + F::new(2.0) * t19635 * t568 + F::new(2.0) * t19644 * t568 + t20032 * t568 - t3882 * t6461 + F::new(4.0) * t5321 * t5326 - F::new(2.0) * t5321 * t5354;
    t20034
}
