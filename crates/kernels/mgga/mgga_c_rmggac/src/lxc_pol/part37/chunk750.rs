//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 750/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk750<F: Float>(t290: F, t70901: F, t2010: F, t7755: F, t31: F, t702: F, t640: F, t7553: F, t7555: F, t2012: F, t7349: F, t2019: F, t7764: F) -> (F, F, F, F, F, F) {
    let t72162 = t290 * t70901;
    let t72164 = t2010 * t7755 * t72162;
    let t72166 = t702 * t31;
    let t72167 = t640 * t72166;
    let t72169 = t7553 * t7555 * t72167;
    let t72170 = F::cast_from(0.43368970657079495312e-4_f64) * t72169;
    let t72171 = t290 * t72166;
    let t72173 = t7349 * t2012 * t72171;
    let t72177 = t2019 * t7764 * t640 * t70901;
    (t72162, t72164, t72170, t72171, t72173, t72177)
}
