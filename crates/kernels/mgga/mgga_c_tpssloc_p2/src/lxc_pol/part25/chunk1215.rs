//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1215/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1215<F: Float>(t82153: F, t218: F, t24234: F, t24325: F, t259: F, t2591: F, t2597: F, t7084: F, t7092: F, t798: F, t82169: F, t82172: F, t82174: F, t82179: F, t82182: F, t82209: F, t84939: F, t9593: F) -> (F, F) {
    let t85101 = F::cast_from(0.27415567780803773942e-2_f64) * t82153;
    let t85126 = -F::cast_from(0.3289868133696452873e-1_f64) * t82169 + F::cast_from(0.49348022005446793095e-1_f64) * t82172 + F::cast_from(0.46058153871750340221e0_f64) * t82174 + F::cast_from(0.9869604401089358619e-1_f64) * t82179 + F::new(12.0) * t9593 * t7092 + F::new(3.0) * t798 * t24234 * t259 + t218 * t84939 * t259 - F::cast_from(0.49348022005446793095e-1_f64) * t82182 + F::new(12.0) * t2597 * t24325 + F::new(3.0) * t2591 * t7084 * t259 - F::cast_from(0.76763589786250567036e0_f64) * t82209;
    (t85101, t85126)
}
