//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2617/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2617<F: Float>(t1215: F, t5011: F, t1222: F, t15765: F, t3242: F, t3448: F, t11728: F, t13969: F, t15630: F, t11678: F, t11722: F, t1174: F, t1177: F, t11825: F, t1227: F, t15560: F, t15617: F, t1653: F, t3490: F, t3509: F, t3578: F, t45086: F, t45102: F, t45162: F, t45197: F, t4582: F, t45993: F, t46006: F, t4733: F, t4972: F, t4987: F, t5030: F, t50879: F) -> (F, F, F, F) {
    let t53176 = t5011 * t1215;
    let t53185 = t15765 * t1222;
    let t53187 = t3448 * t3242;
    let t53220 = t11728 * t13969 * t15630;
    let t53236 = -t1174 * t1177 * t50879 / F::new(12.0) - t3490 * t15617 / F::new(256.0) - t1227 * t4582 * t4972 * t46006 / F::new(768.0) + F::new(5.0) / F::new(13824.0) * t1227 * t4582 * t4987 * t45993 - t53220 / F::new(256.0) + t45086 / F::new(2304.0) - t45162 * t15560 / F::new(768.0) - t11678 * t3578 * t4733 * t3509 / F::new(768.0) - t45197 * t3578 * t1653 * t11722 / F::new(768.0) - t11825 * t5030 / F::new(1536.0) + t45102 / F::new(4608.0);
    (t53176, t53185, t53187, t53236)
}
