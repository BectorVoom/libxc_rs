//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 669/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk669<F: Float>(t598: F, t6924: F, t213: F, t1307: F, t1998: F, t236: F, t1995: F, t6597: F, t133: F, t1999: F, t6600: F, t1996: F, t6604: F) -> (F, F, F, F, F, F, F, F) {
    let t6925 = t598 * t6924;
    let t6926 = t6925 * t213;
    let t6928 = t1998 * t236 * t1307;
    let t6929 = t6926 * t6928;
    let t6931 = t6597 * t1995;
    let t6932 = t6931 * t133;
    let t6933 = t6600 * t1999;
    let t6934 = t6932 * t6933;
    let t6936 = t1996 * t6604;
    (t6925, t6926, t6928, t6929, t6931, t6933, t6934, t6936)
}
