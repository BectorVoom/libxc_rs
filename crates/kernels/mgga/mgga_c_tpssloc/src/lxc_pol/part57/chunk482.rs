//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 482/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk482<F: Float>(t3: F, t6470: F, t1401: F, t1458: F, t3941: F, t5371: F, t5456: F, t5493: F, t577: F, t1862: F, t33: F, t2240: F, t38: F, t43: F, t625: F, t111: F, t1868: F) -> (F, F, F, F, F, F, F) {
    let t6471 = t3 * t6470;
    let t6483 = 0.45e1 * t6470 * t577 + 27.0 * t5371 * t1458 + 27.0 * t3941 * t5456 + 0.135e2 * t1401 * t5493;
    let t6489 = t33 * t1862;
    let t6490 = t2240 * t6489;
    let t6500 = t38 * t43;
    let t6503 = 8.0 / 3.0 * t625;
    let t6517 = t1868 * t111;
    (t6471, t6483, t6489, t6490, t6500, t6503, t6517)
}
