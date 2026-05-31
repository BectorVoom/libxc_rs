//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1184/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1184<F: Float>(t15540: F, t4582: F, t12648: F, t4987: F, t13969: F, t4983: F, t3515: F, t486: F, t5011: F, t4978: F, t11709: F, t11738: F, t11814: F, t11825: F, t1213: F, t1227: F, t15524: F, t15527: F, t15531: F, t15535: F, t1737: F, t1748: F, t3490: F, t3506: F, t3531: F, t3536: F, t4980: F, t4989: F, t5014: F, t5024: F) -> (F, F) {
    let t15541 = t4582 * t15540;
    let t15544 = t4987 * t12648;
    let t15545 = t4582 * t15544;
    let t15548 = t13969 * t4983;
    let t15550 = t3515 * t15548 / F::cast_from(2304.0_f64);
    let t15553 = t486 * t5011;
    let t15554 = t15553 * t4978;
    let t15555 = t4582 * t15554;
    let t15558 = t5024 * t3531 / F::cast_from(432.0_f64) - t11825 * t1748 / F::cast_from(4608.0_f64) + t11814 * t1737 / F::cast_from(3072.0_f64) + t3536 * t5014 / F::cast_from(1536.0_f64) + t15524 + t1213 * t15527 / F::cast_from(3072.0_f64) - t3515 * t15531 / F::cast_from(3072.0_f64) + t11738 * t15535 / F::cast_from(3072.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t3490 * t4989 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t1227 * t15541 + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1227 * t15545 - t15550 + t11709 * t4980 / F::cast_from(768.0_f64) + t3506 * t15555 / F::cast_from(768.0_f64);
    (t15553, t15558)
}
