//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1200/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1200<F: Float>(t19743: F, t19744: F, t5250: F, t5287: F, t5348: F, t1336: F, t16047: F, t19654: F, t19658: F, t19661: F, t19668: F, t19674: F, t19733: F, t19736: F, t19740: F, t3777: F, t5234: F, t5334: F, t5336: F, t5349: F, t6448: F, t6451: F, t6454: F, t6456: F) -> F {
    let t19745 = t19743 * t19744;
    let t19748 = t19743 * t5250;
    let t19752 = t5348 * t5287;
    let t19755 = -t1336 * t19658 + F::cast_from(2.0_f64) * t1336 * t19668 - t1336 * t19674 - t1336 * t19733 - F::cast_from(2.0_f64) * t1336 * t19752 - F::cast_from(6.0_f64) * t16047 * t19745 + F::cast_from(4.0_f64) * t19654 * t5336 + F::cast_from(2.0_f64) * t19661 * t5334 + F::cast_from(4.0_f64) * t19736 * t5334 + F::cast_from(4.0_f64) * t19740 * t5334 + F::cast_from(6.0_f64) * t19748 * t5334 + F::cast_from(2.0_f64) * t3777 * t6448 - F::cast_from(2.0_f64) * t3777 * t6451 - t3777 * t6454 - t3777 * t6456 - F::cast_from(2.0_f64) * t5234 * t5349;
    t19755
}
