//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 821/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk821<F: Float>(t74705: F, t71033: F, t74662: F, t74667: F, t74670: F, t74677: F, t77143: F, t77148: F, t77150: F, t77154: F, t77155: F, t77156: F, t77157: F, t77158: F, t77159: F, t77160: F, t77161: F) -> (F,) {
    let t77162 = 0.5255791827870410156e-5 * t74705;
    let t77163 = -t71033 + 0.35038612185802734376e-6 * t74662 + 0.8759653046450683594e-6 * t74667 - t74670 - t77143 + 0.76860658247009135557e-5 * t74677 - t77148 + t77150 + t77154 - t77155 + t77156 - t77157 - t77158 + t77159 + t77160 + t77161 - t77162;
    (t77163,)
}
