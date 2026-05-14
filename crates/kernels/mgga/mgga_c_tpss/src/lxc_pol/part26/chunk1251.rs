//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1251/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1251<F: Float>(t4715: F, t782: F, t4758: F, t8279: F, t4630: F, t645: F, t1232: F, t17785: F, t1268: F, t5366: F, t5371: F, t1206: F, t5451: F, t1625: F, t4519: F, t4706: F, t821: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t44960 = t4715 * t782;
    let t44994 = t4758 * t782;
    let t45241 = t4715 * t8279;
    let t50656 = t4630 * t645;
    let t51545 = t17785 * t1232;
    let t51622 = t5366 * t1268;
    let t51631 = t5371 * t1268;
    let t51635 = t5451 * t1206;
    let t51642 = t1625 * t4519;
    let t51664 = t5451 * t1268;
    let t51780 = t4706 * t821;
    (t44960, t44994, t45241, t50656, t51545, t51622, t51631, t51635, t51642, t51664, t51780)
}
