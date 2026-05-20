//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 888/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk888<F: Float>(t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11165: F, t11174: F, t11230: F, t11233: F, t11245: F, t11259: F, t11261: F, t11266: F) -> F {
    let t11398 = -F::new(0.82785e-1) * t11230 + F::new(0.49671e0) * t11233 + F::cast_from(0.40256666666666666668e0_f64) * t11137 + F::cast_from(0.20128333333333333333e0_f64) * t11139 - F::cast_from(0.60385000000000000001e0_f64) * t11141 - F::cast_from(0.30192500000000000001e0_f64) * t11143 + F::cast_from(0.33547222222222222222e0_f64) * t11150 - F::new(0.12077e1) * t11156 + F::new(0.181155e1) * t11165 + F::new(0.301925e0) * t11174 - F::cast_from(0.412621875e-1_f64) * t11245 + F::new(0.258925e1) * t11259 + F::new(0.16504875e0) * t11261 + F::new(0.19419375e1) * t11266;
    t11398
}
