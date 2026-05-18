//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1284/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1284<F: Float>(t2693: F, t7503: F, t25132: F, t81882: F, t7500: F, t81911: F, t25064: F, t81902: F, t7521: F, t81632: F, t22690: F, t23171: F, t25319: F) -> (F, F, F, F, F, F) {
    let t87403 = t7503 * t2693;
    let t87405 = t81882 * t25132;
    let t87432 = t81911 * t7500;
    let t87445 = t81902 * t25064;
    let t87635 = t81632 * t7521;
    let t87653 = t23171 * t22690 * t25319;
    (t87403, t87405, t87432, t87445, t87635, t87653)
}
