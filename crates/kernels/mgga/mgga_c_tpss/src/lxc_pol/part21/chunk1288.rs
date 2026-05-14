//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1288/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1288<F: Float>(t10672: F, t215: F, t63993: F, t61087: F, t61081: F, t61089: F, t63971: F, t63974: F, t63975: F, t63978: F, t63979: F, t63981: F, t63984: F, t63987: F, t63991: F, t63927: F, t63947: F, t63970: F) -> (F,) {
    let t63995 = t63993 * t215 * t10672;
    let t63998 = 119.0 / 864.0 * t61087;
    let t64000 = -5.0 / 64.0 * t63971 + t63974 - t63975 / 1536.0 - t63978 + 5.0 / 192.0 * t63979 + 5.0 / 384.0 * t63981 + t63984 / 8.0 + t63987 / 16.0 - t63991 - t63995 / 4.0 + 7.0 / 576.0 * t61081 - t63998 - 35.0 / 576.0 * t61089;
    let t64002 = t63927 + t63947 + t63970 + t64000;
    (t64002,)
}
