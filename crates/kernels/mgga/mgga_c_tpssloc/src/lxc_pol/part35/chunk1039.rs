//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1039/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1039<F: Float>(t1615: F, t5914: F, t1060: F, t21594: F, t381: F, t21390: F, t11048: F, t1625: F, t5872: F, t3188: F, t11060: F, t11066: F) -> (F, F, F, F, F, F, F) {
    let t21626 = t5914 * t1615;
    let t21627 = t21626 * t1060;
    let t21634 = t381 * t21594;
    let t21635 = t21634 * t1060;
    let t21637 = t381 * t21390;
    let t21638 = t21637 * t11048;
    let t21643 = t1625 * t5872;
    let t21644 = t21643 * t3188;
    let t21647 = t21637 * t11060;
    let t21650 = t21637 * t11066;
    (t21627, t21635, t21638, t21643, t21644, t21647, t21650)
}
