//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1181/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1181<F: Float>(t3536: F, t4997: F, t248: F, t3570: F, t5012: F, t1213: F, t3535: F, t5018: F, t1202: F, t5023: F, t1742: F, t3036: F) -> (F, F, F, F, F) {
    let t15490 = t3536 * t4997 / F::cast_from(2304.0_f64);
    let t15492 = t248 * t3570 * t5012;
    let t15494 = t1213 * t15492 / F::cast_from(2304.0_f64);
    let t15495 = t3535 * t5018;
    let t15498 = t1202 * t5023;
    let t15501 = t1742 * t3036;
    (t15490, t15494, t15495, t15498, t15501)
}
