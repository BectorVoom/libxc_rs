//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1574/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1574<F: Float>(t1216: F, t5971: F, t11668: F, t1090: F, t6225: F, t3578: F, t11697: F, t6191: F, t3577: F, t248: F, t3570: F, t6219: F) -> (F, F, F, F) {
    let t18363 = t5971 * t1216;
    let t18364 = t11668 * t18363;
    let t18367 = t6225 * t1090;
    let t18368 = t3578 * t18367;
    let t18371 = t11697 * t6191;
    let t18372 = t3577 * t18371;
    let t18375 = t248 * t3570 * t6219;
    (t18364, t18368, t18372, t18375)
}
