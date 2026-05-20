//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1554/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1554<F: Float>(t1100: F, t11258: F, t1107: F, t410: F, t417: F, t11244: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11165: F, t11174: F, t11230: F, t11233: F, t11245: F) -> (F, F, F, F, F) {
    let t11259 = t1100 * t11258;
    let t11261 = t1107 * t11258;
    let t11265 = F::new(1.0) / t410 / t417 / F::new(4.0);
    let t11266 = t11265 * t11244;
    let t11268 = -F::cast_from(0.82156666666666666668e-1_f64) * t11230 + F::cast_from(0.49293999999999999999e0_f64) * t11233 + F::cast_from(0.39862222222222222223e0_f64) * t11137 + F::cast_from(0.19931111111111111111e0_f64) * t11139 - F::cast_from(0.59793333333333333333e0_f64) * t11141 - F::cast_from(0.29896666666666666667e0_f64) * t11143 + F::cast_from(0.33218518518518518518e0_f64) * t11150 - F::cast_from(0.11958666666666666667e1_f64) * t11156 + F::new(0.17938e1) * t11165 + F::cast_from(0.29896666666666666667e0_f64) * t11174 - F::new(0.76790625e-1) * t11245 + F::new(0.1898925e1) * t11259 + F::new(0.3071625e0) * t11261 + F::cast_from(0.142419375e1_f64) * t11266;
    (t11259, t11261, t11265, t11266, t11268)
}
