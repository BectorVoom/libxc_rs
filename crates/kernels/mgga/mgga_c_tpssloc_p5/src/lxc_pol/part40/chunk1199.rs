//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1199/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1199<F: Float>(t1226: F, t6169: F, t486: F, t6218: F, t4978: F, t4582: F, t1216: F, t17635: F, t4987: F, t4977: F, t5012: F, t11836: F, t1218: F, t1227: F, t1232: F, t15495: F, t15727: F, t15731: F, t15735: F, t15745: F, t1737: F, t19033: F, t19041: F, t19047: F, t3506: F, t3515: F, t3536: F, t4989: F, t5024: F, t6221: F) -> F {
    let t19051 = t6169 * t1226;
    let t19056 = t486 * t6218;
    let t19057 = t19056 * t4978;
    let t19058 = t4582 * t19057;
    let t19061 = t19056 * t1216;
    let t19062 = t4582 * t19061;
    let t19067 = t4987 * t17635;
    let t19068 = t4582 * t19067;
    let t19071 = t4977 * t5012;
    let t19072 = t4582 * t19071;
    let t19075 = -F::cast_from(19.0_f64) / F::cast_from(2592.0_f64) * t19033 * t1232 + t15727 / F::cast_from(81.0_f64) - t15731 / F::cast_from(6912.0_f64) + t15735 / F::cast_from(10368.0_f64) - t19041 / F::cast_from(6912.0_f64) + t3536 * t6221 / F::cast_from(3072.0_f64) + t19047 * t1218 / F::cast_from(3072.0_f64) + t15745 + t11836 / F::cast_from(1296.0_f64) - t19051 * t1232 / F::cast_from(4608.0_f64) - t15495 * t1737 / F::cast_from(288.0_f64) + t3506 * t19058 / F::cast_from(1536.0_f64) - t3515 * t19062 / F::cast_from(3072.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t5024 * t4989 + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1227 * t19068 - t3515 * t19072 / F::cast_from(1536.0_f64);
    t19075
}
