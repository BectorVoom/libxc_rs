//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1467/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1467<F: Float>(t1174: F, t11760: F, t135: F, t11147: F, t3439: F, t11719: F, t11724: F, t11728: F, t11734: F, t11770: F, t11814: F, t1214: F, t1216: F, t1227: F, t1230: F, t1232: F, t15620: F, t248: F, t3496: F, t3506: F, t3508: F, t3511: F, t3515: F, t3518: F, t39097: F, t43757: F, t44668: F, t44873: F, t44879: F, t44886: F, t44890: F, t44894: F, t44896: F, t44904: F, t44906: F, t44918: F, t44929: F, t44932: F, t4582: F, t974: F) -> F {
    let t44936 = t1174 * t135 * t11760;
    let t44938 = t3439 * t11147;
    let t44943 = -t3515 * t4582 * t44879 * t1216 / F::new(768.0) - t44886 / F::new(2304.0) - t44890 / F::new(1152.0) + t44894 / F::new(2304.0) + t44896 * t11724 / F::new(128.0) - t1227 * t248 * t1230 * t43757 / F::new(768.0) + t44904 / F::new(192.0) + F::new(3.0) / F::new(256.0) * t11719 * t4582 * t44873 * t44906 - t11734 * t11770 / F::new(256.0) + t3506 * t248 * t1214 * t44668 * t3508 / F::new(512.0) - t44918 * t1232 / F::new(1152.0) - F::new(3.0) / F::new(256.0) * t11728 * t4582 * t44873 * t15620 + t11814 * t3496 / F::new(512.0) + t44929 * t3511 / F::new(256.0) - t44932 * t3518 / F::new(512.0) + t44936 / F::new(27.0) + t1174 * t974 * t44938 * t39097 / F::new(6.0);
    t44943
}
