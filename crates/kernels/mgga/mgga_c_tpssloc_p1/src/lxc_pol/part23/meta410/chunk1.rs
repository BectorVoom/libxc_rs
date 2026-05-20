//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1226/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1226<F: Float>(t15568: F, t5064: F, t1227: F, t248: F, t45046: F, t5971: F, t3032: F, t65253: F, t3505: F, t3514: F, t1174: F, t6187: F, t698: F) -> (F, F, F, F, F) {
    let t65884 = t5064 * t15568;
    let t65935 = t1227 * t248 * t45046 * t5971;
    let t65962 = t65253 * t3032;
    let t65963 = t65962 * t3505;
    let t65966 = t65962 * t3514;
    let t66015 = t1174 * t698 * t6187;
    (t65884, t65935, t65963, t65966, t66015)
}
