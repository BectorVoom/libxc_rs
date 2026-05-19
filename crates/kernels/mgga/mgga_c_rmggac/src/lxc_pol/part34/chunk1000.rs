//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1000/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1000<F: Float>(t14567: F, t1562: F, t75186: F, t75192: F, t75195: F, t75198: F, t75202: F, t75206: F, t75210: F, t75214: F, t75217: F, t75221: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t77497 = t1562 * t14567;
    let t77502 = F::cast_from(0.85129199786595678799e-5_f64) * t75186;
    let t77503 = F::cast_from(0.85129199786595678799e-5_f64) * t75192;
    let t77504 = F::cast_from(0.2553875993597870364e-4_f64) * t75195;
    let t77505 = F::cast_from(0.3830813990396805546e-4_f64) * t75198;
    let t77506 = F::cast_from(0.72732431077987577947e-1_f64) * t75202;
    let t77507 = F::cast_from(0.30487649791575028312e-3_f64) * t75206;
    let t77508 = F::cast_from(0.30487649791575028312e-3_f64) * t75210;
    let t77509 = F::cast_from(0.30487649791575028312e-3_f64) * t75214;
    let t77510 = F::cast_from(0.14967802127329760705e-1_f64) * t75217;
    let t77511 = F::cast_from(0.85129199786595678799e-5_f64) * t75221;
    (t77497, t77502, t77503, t77504, t77505, t77506, t77507, t77508, t77509, t77510, t77511)
}
