//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 720/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk720<F: Float>(t3140: F, t3144: F, t9086: F, t1587: F, t234: F, t3157: F, t69064: F, t69069: F, t13949: F, t6355: F, t13975: F, t38530: F, t27: F, t9169: F, t16058: F, t69609: F) -> (F, F, F, F, F, F, F) {
    let t74994 = t9086 * t3140 * t3144;
    let t74996 = t234 * t1587;
    let t74997 = t74996 * t3157;
    let t75002 = 0.39726959900411316772e-4 * t69064;
    let t75003 = 0.19863479950205658386e-4 * t69069;
    let t75005 = 0.5987120850931904282e-1 * t6355 * t13949;
    let t75006 = t38530 * t13975;
    let t75008 = t27 * t9169;
    let t75010 = t69609 * t16058 * t75008;
    (t74994, t74997, t75002, t75003, t75005, t75006, t75010)
}
