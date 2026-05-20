//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1621/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1621<F: Float>(t1226: F, t6169: F, t486: F, t6218: F, t4978: F, t4582: F, t1216: F, t17635: F, t4987: F, t4977: F, t5012: F, t11836: F, t1218: F, t1227: F, t1232: F, t15495: F, t15727: F, t15731: F, t15735: F, t15745: F, t1737: F, t19033: F, t19041: F, t19047: F, t3506: F, t3515: F, t3536: F, t4989: F, t5024: F, t6221: F) -> (F, F, F, F, F) {
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
    let t19075 = -F::new(19.0) / F::new(2592.0) * t19033 * t1232 + t15727 / F::new(81.0) - t15731 / F::new(6912.0) + t15735 / F::new(10368.0) - t19041 / F::new(6912.0) + t3536 * t6221 / F::new(3072.0) + t19047 * t1218 / F::new(3072.0) + t15745 + t11836 / F::new(1296.0) - t19051 * t1232 / F::new(4608.0) - t15495 * t1737 / F::new(288.0) + t3506 * t19058 / F::new(1536.0) - t3515 * t19062 / F::new(3072.0) - F::new(5.0) / F::new(1296.0) * t5024 * t4989 + F::new(5.0) / F::new(13824.0) * t1227 * t19068 - t3515 * t19072 / F::new(1536.0);
    (t19058, t19062, t19068, t19072, t19075)
}
