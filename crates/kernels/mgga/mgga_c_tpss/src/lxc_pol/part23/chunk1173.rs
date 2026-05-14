//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1173/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1173<F: Float>(t1148: F, t6032: F, t6034: F, t6016: F, t6025: F, t1880: F, t3144: F, t1138: F, t18169: F, t1883: F, t3048: F, t2785: F, t3053: F, t3054: F, t1107: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19130 = t6032 * t1148;
    let t19131 = t19130 * t6034;
    let t19135 = t6025 * t6016 * t1148;
    let t19139 = t6025 * t1880 * t3144;
    let t19142 = t18169 * t1138;
    let t19143 = t1883 * t19142;
    let t19144 = t3048 * t1880;
    let t19145 = t3053 * t2785;
    let t19146 = t19145 * t3054;
    let t19147 = t19144 * t19146;
    let t19150 = t1107 * t6016;
    (t19131, t19135, t19139, t19142, t19143, t19144, t19145, t19146, t19147, t19150)
}
