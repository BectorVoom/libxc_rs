//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3181/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3181<F: Float>(t1009: F, t18571: F, t1011: F, t1212: F, t3032: F, t65253: F, t3505: F, t3514: F, t1218: F, t1227: F, t15455: F, t15541: F, t15545: F, t15656: F, t18590: F, t18594: F, t18955: F, t19047: F, t3490: F, t3496: F, t3511: F, t3518: F, t4582: F, t4972: F, t5005: F, t52817: F, t52845: F, t52859: F, t61798: F) -> (F, F) {
    let t65955 = t18571 * t1009;
    let t65957 = t65955 * t1011 * t1212;
    let t65962 = t65253 * t3032;
    let t65963 = t65962 * t3505;
    let t65966 = t65962 * t3514;
    let t65990 = t65957 * t1218 / F::cast_from(1536.0_f64) + t19047 * t3496 / F::cast_from(3072.0_f64) + t65963 * t3511 / F::cast_from(1536.0_f64) - t65966 * t3518 / F::cast_from(3072.0_f64) + t52817 / F::cast_from(576.0_f64) + t52845 / F::cast_from(432.0_f64) - F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t3490 * t18955 - F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t5005 * t15455 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t5005 * t15541 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t5005 * t15545 + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t5005 * t15656 - t3490 * t18590 / F::cast_from(576.0_f64) - t1227 * t4582 * t4972 * t61798 / F::cast_from(1152.0_f64) - t3490 * t18594 / F::cast_from(384.0_f64) + t52859 / F::cast_from(1152.0_f64);
    (t65955, t65990)
}
