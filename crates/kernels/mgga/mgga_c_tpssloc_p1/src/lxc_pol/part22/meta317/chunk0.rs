//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1499/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1499<F: Float>(t11539: F, t4724: F, t1174: F, t13969: F, t4983: F, t3515: F, t1742: F, t478: F, t3068: F, t1244: F) -> (F, F, F, F, F, F, F) {
    let t15522 = t11539 * t4724;
    let t15524 = t1174 * t15522 / F::cast_from(324.0_f64);
    let t15548 = t13969 * t4983;
    let t15550 = t3515 * t15548 / F::cast_from(2304.0_f64);
    let t15567 = t478 * t1742;
    let t15568 = t15567 * t3068;
    let t15569 = t1244 * t15568;
    (t15522, t15524, t15548, t15550, t15567, t15568, t15569)
}
