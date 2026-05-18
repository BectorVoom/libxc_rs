//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1020/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1020<F: Float>(t14276: F, t21259: F, t21263: F, t21265: F, t21267: F, t21270: F, t21302: F, t21305: F, t21306: F, t21309: F, t21312: F, t21317: F, t21320: F, t21321: F, t21336: F, t21348: F, t21360: F, t2861: F, t2886: F, t2905: F, t2930: F, t311: F, t5743: F) -> F {
    let t21363 = -F::new(6.0) * t14276 * t5743 + F::new(6.0) * t2886 * t21259 - t21263 - t21265 - t21267 + t21270 - t21302 - t21305 + F::new(0.96491876992155210402e2) * t2886 * t21306 - F::new(0.35089341735807877242e1) * t2905 * t21309 + F::new(0.51947577317044391277e2) * t2930 * t21312 + t21317 - t21320 - F::new(6.0) * t2861 * t21321 + t21336 - F::new(0.19751673498613801407e-1) * t21348 - F::new(0.310907e-1) * t21360 * t311;
    t21363
}
