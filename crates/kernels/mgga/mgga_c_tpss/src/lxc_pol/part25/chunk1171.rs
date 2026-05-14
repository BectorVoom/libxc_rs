//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1171/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1171<F: Float>(t1232: F, t5407: F, t5380: F, t10089: F, t13943: F, t3205: F, t1364: F, t2436: F, t4715: F, t782: F, t4758: F, t8279: F, t4630: F, t645: F, t17785: F, t1268: F, t5366: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t43101 = t5407 * t1232;
    let t43602 = t5380 * t1232;
    let t43710 = t5380 * t10089;
    let t44034 = t13943 * t3205;
    let t44169 = t2436 * t1364;
    let t44960 = t4715 * t782;
    let t44994 = t4758 * t782;
    let t45241 = t4715 * t8279;
    let t50656 = t4630 * t645;
    let t51545 = t17785 * t1232;
    let t51622 = t5366 * t1268;
    (t43101, t43602, t43710, t44034, t44169, t44960, t44994, t45241, t50656, t51545, t51622)
}
