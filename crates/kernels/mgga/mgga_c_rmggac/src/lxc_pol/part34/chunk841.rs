//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 841/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk841<F: Float>(t14567: F, t1562: F, t75186: F, t75192: F, t75195: F, t75198: F, t75202: F, t75206: F, t75210: F, t75214: F, t75217: F, t75221: F, t75225: F, t75166: F, t75169: F, t75174: F, t75180: F, t75184: F) -> (F,) {
    let t77497 = t1562 * t14567;
    let t77502 = 0.85129199786595678799e-5 * t75186;
    let t77503 = 0.85129199786595678799e-5 * t75192;
    let t77504 = 0.2553875993597870364e-4 * t75195;
    let t77505 = 0.3830813990396805546e-4 * t75198;
    let t77506 = 0.72732431077987577947e-1 * t75202;
    let t77507 = 0.30487649791575028312e-3 * t75206;
    let t77508 = 0.30487649791575028312e-3 * t75210;
    let t77509 = 0.30487649791575028312e-3 * t75214;
    let t77510 = 0.14967802127329760705e-1 * t75217;
    let t77511 = 0.85129199786595678799e-5 * t75221;
    let t77512 = 0.2553875993597870364e-4 * t75225;
    let t77513 = 0.10511583655740820313e-5 * t75166 - 0.52557918278704101561e-5 * t75169 - 0.2363e1 * t77497 + 0.29085809927086856923e-4 * t75174 + 0.72714524817717142308e-5 * t75180 - 0.72714524817717142308e-5 * t75184 + t77502 + t77503 + t77504 - t77505 - t77506 + t77507 + t77508 + t77509 - t77510 - t77511 + t77512;
    (t77513,)
}
