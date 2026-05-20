//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 896/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk896<F: Float>(t21158: F, t21193: F, t932: F, t10813: F, t21114: F, t21089: F, t2932: F, t10542: F, t10545: F, t21120: F, t21124: F, t21128: F, t21132: F, t21136: F, t21140: F, t21142: F, t21144: F, t21147: F, t21150: F, t21153: F, t21156: F) -> (F, F, F, F) {
    let t21194 = t21158 + t21193;
    let t21195 = t21194 * t932;
    let t21198 = t21114 * t10813;
    let t21207 = t21089 * t2932;
    let t21222 = F::new(0.16557e0) * t21120 - F::cast_from(0.60384999999999999999e0_f64) * t21124 + F::new(0.181155e1) * t21128 - F::cast_from(0.36793333333333333333e-1_f64) * t21132 - F::new(0.82785e-1) * t21136 - F::new(0.49671e0) * t21140 - F::new(0.3883875e1) * t21142 + F::cast_from(0.247573125e0_f64) * t21144 - t10542 - t10545 - F::cast_from(0.33547222222222222222e0_f64) * t21147 + F::new(0.12077e1) * t21150 - F::new(0.181155e1) * t21153 - F::new(0.301925e0) * t21156;
    (t21195, t21198, t21207, t21222)
}
