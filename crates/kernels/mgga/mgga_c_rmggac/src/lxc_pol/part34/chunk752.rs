//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 752/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk752<F: Float>(t72019: F, t797: F, t333: F, t4669: F, t71949: F, t305: F, t71637: F, t14506: F, t2085: F, t69518: F, t69521: F, t14584: F, t504: F) -> (F, F, F, F, F, F, F) {
    let t72023 = t797 * t72019;
    let t72027 = t4669 * t71949 * t333;
    let t72037 = t305 * t71637;
    let t72038 = F::cast_from(0.14635184302277988245e0_f64) * t72037;
    let t72062 = t14506 * t2085;
    let t72087 = F::cast_from(0.34547904762044099522e0_f64) * t69518;
    let t72088 = F::cast_from(0.50557909407869413937e0_f64) * t69521;
    let t72109 = t504 * t14584;
    (t72023, t72027, t72038, t72062, t72087, t72088, t72109)
}
