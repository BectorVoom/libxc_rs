//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1001/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1001<F: Float>(t75225: F, t75166: F, t75169: F, t75174: F, t75180: F, t75184: F, t77497: F, t77502: F, t77503: F, t77504: F, t77505: F, t77506: F, t77507: F, t77508: F, t77509: F, t77510: F, t77511: F) -> F {
    let t77512 = F::cast_from(0.2553875993597870364e-4_f64) * t75225;
    let t77513 = F::cast_from(0.10511583655740820313e-5_f64) * t75166 - F::cast_from(0.52557918278704101561e-5_f64) * t75169 - F::new(0.2363e1) * t77497 + F::cast_from(0.29085809927086856923e-4_f64) * t75174 + F::cast_from(0.72714524817717142308e-5_f64) * t75180 - F::cast_from(0.72714524817717142308e-5_f64) * t75184 + t77502 + t77503 + t77504 - t77505 - t77506 + t77507 + t77508 + t77509 - t77510 - t77511 + t77512;
    t77513
}
