//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2302/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2302<F: Float>(t10126: F, t10134: F, t12854: F, t12895: F, t12899: F, t13196: F, t13471: F, t13487: F, t16596: F, t1877: F, t2379: F, t2522: F, t2523: F, t2553: F, t2752: F, t39249: F, t39256: F, t39373: F, t39397: F, t39463: F, t39468: F, t39472: F, t39476: F, t39529: F, t40689: F, t40721: F, t40779: F, t40784: F, t40790: F, t40793: F, t4119: F, t41254: F, t41258: F, t41262: F, t4255: F, t4307: F, t4310: F, t4314: F, t4315: F, t46120: F, t46126: F, t46129: F, t46131: F, t46133: F, t46135: F, t46138: F, t46145: F, t46152: F, t46194: F, t46195: F, t46197: F, t46219: F, t46228: F, t46232: F, t46257: F, t46281: F, t46294: F, t46298: F, t46303: F, t46309: F, t46311: F, t46324: F, t46340: F, t46373: F, t46377: F, t46384: F, t46385: F, t46386: F, t46389: F, t46450: F, t47644: F, t47651: F, t868: F, t9470: F, t9516: F, t9616: F) -> F {
    let t47655 = -t46133 + t46135 - t46377 - t39256 + t39373 + t46232 - t46129 - t46131 + t40689 - t46311 - t39472 - t46126 - t39468 + t47644 + t40793 + t40790 - t39397 + t46145 + t46340 + t46281 - t46228 - t46309 + t46389 + t46257 - t40779 + t46219 + t46303 + t39463 - t40721 + t46120 - t46384 - t46385 - t46386 + t46152 - t39476 + t46450 + t46194 + t46195 + t46197 + t46373 - t41258 + t47651 - t41262 + t40784 + t46324 - F::new(18.0) * t2522 * t12854 * t13487 + F::new(18.0) * t4314 * t10126 * t4255 + F::new(18.0) * t4314 * t4310 * t9616 + F::new(18.0) * t4314 * t12895 * t2379 - F::new(9.0) * t2522 * t9470 * t4119 + F::new(18.0) * t4314 * t2523 * t13196 + F::new(6.0) * t4314 * t4315 * t9516 - F::new(18.0) * t4314 * t4307 * t46298 - F::new(9.0) * t2522 * t10134 * t16596 + F::new(18.0) * t4314 * t12899 * t2553 - t39249 + t46138 + t41254 - F::new(3.0) * t1877 * t13471 * t2752 * t868 + t46294 - t39529;
    t47655
}
