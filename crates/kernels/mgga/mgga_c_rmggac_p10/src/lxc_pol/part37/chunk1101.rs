//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1101/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1101<F: Float>(t305: F, t69146: F, t76171: F, t76180: F, t76182: F, t76184: F, t77860: F, t77863: F, t77864: F, t77868: F, t77869: F, t77870: F, t77873: F, t80398: F) -> F {
    let t80421 = -F::cast_from(0.15531404553111930707e-1_f64) * t76171 - t77860 + t77863 + t77864 + F::cast_from(0.93188427318671584242e-2_f64) * t76180 - F::cast_from(0.15531404553111930707e-1_f64) * t76182 - F::cast_from(0.62125618212447722828e-2_f64) * t76184 + t77868 - t77869 - t77870 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t80398 - t77873 - t69146;
    t80421
}
