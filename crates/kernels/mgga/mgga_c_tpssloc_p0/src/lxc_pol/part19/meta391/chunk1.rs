//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1472/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1472<F: Float>(t3584: F, t676: F, t1227: F, t248: F, t3243: F, t11159: F, t11665: F, t11668: F, t11678: F, t11684: F, t11721: F, t1174: F, t1177: F, t11805: F, t1214: F, t1216: F, t15620: F, t15661: F, t15708: F, t2250: F, t3247: F, t3490: F, t3508: F, t3577: F, t3578: F, t42374: F, t43723: F, t44699: F, t45002: F, t45007: F, t45009: F, t45013: F, t45015: F, t45020: F, t45027: F, t45030: F, t45037: F, t45044: F, t4582: F, t4987: F) -> F {
    let t45046 = t676 * t3584;
    let t45049 = t1227 * t248 * t45046 * t3243;
    let t45066 = -t11665 * t11684 / F::new(384.0) + t45002 / F::new(2592.0) - t1174 * t1177 * t43723 / F::new(36.0) + t45007 / F::new(1152.0) - t45009 / F::new(576.0) - t45013 / F::new(1728.0) - t45015 / F::new(288.0) + t45020 / F::new(2592.0) + F::new(5.0) / F::new(3456.0) * t1227 * t4582 * t4987 * t42374 - t45027 / F::new(288.0) - F::new(3.0) / F::new(256.0) * t45030 * t248 * t1214 * t44699 * t11721 + F::new(7.0) / F::new(1536.0) * t45037 * t248 * t1214 * t44699 * t3508 - F::new(5.0) / F::new(972.0) * t45044 - F::new(5.0) / F::new(10368.0) * t45049 - t3577 * t3578 * t3247 * t2250 * t15708 / F::new(192.0) + F::new(5.0) / F::new(1152.0) * t3577 * t11668 * t1216 * t11159 - t11678 * t3578 * t15620 * t15661 / F::new(192.0) - t3490 * t11805 / F::new(1152.0);
    t45066
}
