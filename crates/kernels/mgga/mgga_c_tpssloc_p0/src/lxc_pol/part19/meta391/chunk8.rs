//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1479/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1479<F: Float>(t11159: F, t11539: F, t1174: F, t1090: F, t11154: F, t11546: F, t11668: F, t11678: F, t11722: F, t11855: F, t11863: F, t1196: F, t1216: F, t1227: F, t3243: F, t3248: F, t3252: F, t3440: F, t3490: F, t3494: F, t3509: F, t3536: F, t3577: F, t3578: F, t39097: F, t39110: F, t42374: F, t43711: F, t43715: F, t43732: F, t45192: F, t45197: F, t45211: F, t45222: F, t45224: F, t4582: F, t4972: F, t974: F) -> F {
    let t45227 = t1174 * t11539 * t11159;
    let t45246 = -t1174 * t974 * t1196 * t39110 / F::new(288.0) - t1174 * t974 * t45192 * t39097 / F::new(12.0) - t45197 * t3578 * t11722 * t1090 / F::new(192.0) - t3577 * t3578 * t3494 * t3252 / F::new(768.0) - t3577 * t3578 * t3494 * t3248 / F::new(384.0) + F::new(5.0) / F::new(1728.0) * t45211 - t3490 * t11863 / F::new(192.0) - t1227 * t4582 * t4972 * t42374 / F::new(576.0) + t3536 * t11855 / F::new(768.0) - t45222 / F::new(36.0) - t45224 / F::new(2304.0) + t45227 / F::new(54.0) + t1174 * t3440 * t43715 / F::new(54.0) - F::new(7.0) / F::new(108.0) * t1174 * t11546 * t43732 + t1174 * t3440 * t43711 / F::new(6.0) + F::new(5.0) / F::new(1152.0) * t11678 * t11668 * t3509 * t3243 + F::new(5.0) / F::new(576.0) * t3577 * t11668 * t1216 * t11154;
    t45246
}
