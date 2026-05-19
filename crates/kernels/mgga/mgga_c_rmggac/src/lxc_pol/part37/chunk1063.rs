//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1063/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1063<F: Float>(t74779: F, t68911: F, t71151: F, t74772: F, t74775: F, t74782: F, t74786: F, t77193: F, t77195: F, t77196: F, t77197: F, t77206: F, t77208: F, t77212: F, t77213: F, t77214: F, t77218: F) -> F {
    let t80158 = F::cast_from(0.65053455985619242964e-5_f64) * t74779;
    let t80160 = -t77193 - F::cast_from(0.1313947956967602539e-5_f64) * t74772 - t77195 - t77196 + t77197 - F::cast_from(0.43798265232253417968e-6_f64) * t74775 - t68911 + t80158 - t74782 + t71151 - t77206 + F::cast_from(0.72714524817717142305e-5_f64) * t74786 - t77208 - t77212 - t77213 + t77214 - t77218;
    t80160
}
