//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1051/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1051<F: Float>(t70905: F, t74228: F, t74235: F, t74238: F, t74243: F, t76931: F, t76932: F, t76935: F, t76937: F, t76939: F, t76940: F, t76941: F, t76942: F, t76943: F, t76946: F, t76947: F, t76948: F) -> F {
    let t80071 = F::cast_from(0.70077224371605468748e-6_f64) * t74228 + t76931 - t76932 + F::cast_from(0.35038612185802734374e-6_f64) * t74235 + t76935 - F::cast_from(0.52557918278704101561e-6_f64) * t74238 + t76937 + F::cast_from(0.76860658247009135562e-5_f64) * t74243 - t76939 - t76940 + t76941 + t76942 - t70905 - t76943 - t76946 - t76947 - t76948;
    t80071
}
