//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1222/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1222<F: Float>(t27863: F, t6534: F, t122917: F, t1873: F, t111: F, t33685: F, t671: F, t8666: F, t96238: F, t116152: F, t120121: F, t123023: F, t123050: F, t123052: F, t123054: F, t123056: F, t1458: F, t31237: F, t31239: F, t31880: F, t33152: F, t33154: F, t4072: F, t8446: F) -> (F, F, F) {
    let t123058 = t27863 * t6534;
    let t123060 = t122917 * t1873;
    let t123062 = t33685 * t111;
    let t123067 = t8666 * t671;
    let t123072 = t96238 * t1873;
    let t123074 = 2.0 * t116152 * t1458 + 2.0 * t123062 * t671 + 2.0 * t123067 * t1458 + 2.0 * t31880 * t4072 + t120121 + t123023 + 2.0 * t123050 + 2.0 * t123052 + 2.0 * t123054 + 2.0 * t123056 + 2.0 * t123058 + 2.0 * t123060 + 2.0 * t123072 + t31237 + t31239 + t33152 + t33154 + t8446;
    (t123062, t123067, t123074)
}
