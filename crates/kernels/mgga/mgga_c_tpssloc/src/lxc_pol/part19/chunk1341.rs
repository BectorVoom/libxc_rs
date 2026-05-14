//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1341/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1341<F: Float>(t11153: F, t1176: F, t11881: F, t45113: F, t11773: F, t1227: F, t13969: F, t11168: F, t1174: F, t3431: F, t3540: F, t3567: F, t11159: F, t11539: F, t1090: F, t11154: F, t11546: F, t11668: F, t11678: F, t11722: F, t11855: F, t11863: F, t1196: F, t1216: F, t3243: F, t3248: F, t3252: F, t3440: F, t3490: F, t3494: F, t3509: F, t3536: F, t3577: F, t3578: F, t39097: F, t39110: F, t42374: F, t43711: F, t43715: F, t43732: F, t4582: F, t4972: F, t974: F) -> (F,) {
    let t45192 = t1176 * t11153;
    let t45197 = t11881 * t45113;
    let t45211 = t1227 * t13969 * t11773;
    let t45222 = t1174 * t3431 * t11168;
    let t45224 = t3567 * t3540;
    let t45227 = t1174 * t11539 * t11159;
    let t45246 = -t1174 * t974 * t1196 * t39110 / 288.0 - t1174 * t974 * t45192 * t39097 / 12.0 - t45197 * t3578 * t11722 * t1090 / 192.0 - t3577 * t3578 * t3494 * t3252 / 768.0 - t3577 * t3578 * t3494 * t3248 / 384.0 + 5.0 / 1728.0 * t45211 - t3490 * t11863 / 192.0 - t1227 * t4582 * t4972 * t42374 / 576.0 + t3536 * t11855 / 768.0 - t45222 / 36.0 - t45224 / 2304.0 + t45227 / 54.0 + t1174 * t3440 * t43715 / 54.0 - 7.0 / 108.0 * t1174 * t11546 * t43732 + t1174 * t3440 * t43711 / 6.0 + 5.0 / 1152.0 * t11678 * t11668 * t3509 * t3243 + 5.0 / 576.0 * t3577 * t11668 * t1216 * t11154;
    (t45246,)
}
