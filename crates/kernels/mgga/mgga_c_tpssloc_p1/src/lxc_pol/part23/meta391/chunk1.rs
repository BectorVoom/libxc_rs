//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1196/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1196<F: Float>(t212: F, t6330: F, t2586: F, t40353: F, t6347: F, t12225: F, t40018: F, t6353: F, t12189: F, t6358: F, t19767: F, t40409: F) -> (F, F, F, F, F) {
    let t56463 = t212 * t6330;
    let t56465 = t2586 * t40353 * t56463;
    let t56467 = t212 * t6347;
    let t56469 = t2586 * t12225 * t56467;
    let t56484 = t40018 * t6353;
    let t56491 = t12189 * t6358;
    let t56535 = t40409 * t19767;
    (t56465, t56469, t56484, t56491, t56535)
}
