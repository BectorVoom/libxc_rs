//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 817/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk817<F: Float>(t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11165: F, t11174: F, t11230: F, t11233: F, t11245: F, t11259: F, t11261: F, t11266: F, t11383: F, t1156: F) -> (F, F) {
    let t11398 = -0.82785e-1 * t11230 + 0.49671e0 * t11233 + 0.40256666666666666668e0 * t11137 + 0.20128333333333333333e0 * t11139 - 0.60385000000000000001e0 * t11141 - 0.30192500000000000001e0 * t11143 + 0.33547222222222222222e0 * t11150 - 0.12077e1 * t11156 + 0.181155e1 * t11165 + 0.301925e0 * t11174 - 0.412621875e-1 * t11245 + 0.258925e1 * t11259 + 0.16504875e0 * t11261 + 0.19419375e1 * t11266;
    let t11399 = t11383 + t11398;
    let t11400 = t11399 * t1156;
    (t11399, t11400)
}
