//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1297/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1297<F: Float>(t27188: F, t6534: F, t121004: F, t1873: F, t121007: F, t33234: F, t23938: F, t7467: F, t26977: F, t26135: F, t7042: F, t120121: F, t120123: F, t120125: F, t120131: F, t120145: F, t120148: F, t121129: F, t2039: F, t22461: F, t31237: F, t31239: F, t33085: F, t7056: F, t7801: F) -> (F, F, F, F, F, F, F, F) {
    let t122734 = t27188 * t6534;
    let t122735 = t121004 * t1873;
    let t122736 = t121007 * t1873;
    let t122737 = t33234 * t6534;
    let t122738 = t23938 * t7467;
    let t122739 = t26977 * t7467;
    let t122740 = t7042 * t26135;
    let t122754 = 2.0 * t120145 * t2039 + 2.0 * t120148 * t2039 + 2.0 * t22461 * t7801 + 2.0 * t33085 * t7056 + t120121 + t120123 + t120125 + t120131 + t121129 + t31237 + t31239;
    (t122734, t122735, t122736, t122737, t122738, t122739, t122740, t122754)
}
