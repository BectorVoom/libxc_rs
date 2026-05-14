//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1126/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1126<F: Float>(t15437: F, t15506: F, t19201: F, t3576: F, t3577: F, t44951: F, t6191: F, t15568: F, t5064: F, t1227: F, t248: F, t45046: F, t5971: F, t3032: F, t65253: F, t3505: F) -> (F, F, F, F, F, F, F) {
    let t65706 = t15437 * t15506;
    let t65815 = t19201 * t3576;
    let t65819 = t3577 * t44951 * t6191;
    let t65884 = t5064 * t15568;
    let t65935 = t1227 * t248 * t45046 * t5971;
    let t65962 = t65253 * t3032;
    let t65963 = t65962 * t3505;
    (t65706, t65815, t65819, t65884, t65935, t65962, t65963)
}
