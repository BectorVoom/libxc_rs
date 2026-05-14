//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1082/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1082<F: Float>(t19731: F, t550: F, t1380: F, t3792: F, t5286: F, t5335: F, t1824: F, t1834: F, t5250: F, t562: F, t6387: F, t12250: F, t1351: F, t5287: F, t5348: F, t1336: F, t16047: F, t19654: F, t19658: F, t19661: F, t19668: F, t19674: F, t3777: F, t5234: F, t5334: F, t5336: F, t5349: F, t6448: F, t6451: F, t6454: F, t6456: F) -> (F, F, F, F, F, F) {
    let t19732 = t19731 * t550;
    let t19733 = t1380 * t19732;
    let t19735 = t3792 * t5286;
    let t19736 = t5335 * t19735;
    let t19739 = t1834 * t1824;
    let t19740 = t19739 * t5250;
    let t19743 = t562 * t6387;
    let t19744 = t12250 * t1351;
    let t19745 = t19743 * t19744;
    let t19748 = t19743 * t5250;
    let t19752 = t5348 * t5287;
    let t19755 = -t1336 * t19658 + 2.0 * t1336 * t19668 - t1336 * t19674 - t1336 * t19733 - 2.0 * t1336 * t19752 - 6.0 * t16047 * t19745 + 4.0 * t19654 * t5336 + 2.0 * t19661 * t5334 + 4.0 * t19736 * t5334 + 4.0 * t19740 * t5334 + 6.0 * t19748 * t5334 + 2.0 * t3777 * t6448 - 2.0 * t3777 * t6451 - t3777 * t6454 - t3777 * t6456 - 2.0 * t5234 * t5349;
    (t19732, t19735, t19739, t19743, t19744, t19755)
}
