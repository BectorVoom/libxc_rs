//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 967/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk967<F: Float>(t75195: F, t75198: F, t75202: F, t75206: F, t75210: F, t75214: F, t75217: F, t75221: F, t75225: F, t75231: F, t75235: F, t3219: F, t9087: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77504 = F::new(0.2553875993597870364e-4) * t75195;
    let t77505 = F::new(0.3830813990396805546e-4) * t75198;
    let t77506 = F::new(0.72732431077987577947e-1) * t75202;
    let t77507 = F::new(0.30487649791575028312e-3) * t75206;
    let t77508 = F::new(0.30487649791575028312e-3) * t75210;
    let t77509 = F::new(0.30487649791575028312e-3) * t75214;
    let t77510 = F::new(0.14967802127329760705e-1) * t75217;
    let t77511 = F::new(0.85129199786595678799e-5) * t75221;
    let t77512 = F::new(0.2553875993597870364e-4) * t75225;
    let t77514 = F::new(0.2553875993597870364e-4) * t75231;
    let t77515 = F::new(0.1702583995731913576e-4) * t75235;
    let t77516 = t9087 * t3219;
    (t77504, t77505, t77506, t77507, t77508, t77509, t77510, t77511, t77512, t77514, t77515, t77516)
}
