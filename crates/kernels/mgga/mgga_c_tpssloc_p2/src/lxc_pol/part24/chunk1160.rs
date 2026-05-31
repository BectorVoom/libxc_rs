//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1160/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1160<F: Float>(t23132: F, t812: F, t849: F, t2707: F, t6621: F, t1891: F, t9223: F, t213: F, t1895: F, t1887: F, t206: F, t22715: F) -> (F, F, F, F, F, F) {
    let t23133 = t812 * t23132;
    let t23134 = t23133 * t849;
    let t23135 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t23134;
    let t23136 = t6621 * t2707;
    let t23138 = t9223 * t1891;
    let t23139 = t23138 * t213;
    let t23140 = t23139 * t1895;
    let t23141 = F::cast_from(0.11304371706359309439e-1_f64) * t23140;
    let t23143 = t22715 * t206 * t1887;
    (t23133, t23135, t23136, t23138, t23141, t23143)
}
