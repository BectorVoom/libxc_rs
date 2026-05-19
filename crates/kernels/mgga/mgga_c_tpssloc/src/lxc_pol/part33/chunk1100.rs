//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1100/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1100<F: Float>(t6589: F, t6597: F, t281: F, t6619: F, t835: F, t812: F, t1891: F, t9223: F, t213: F, t1895: F, t1887: F, t206: F, t22715: F) -> (F, F, F, F, F, F, F) {
    let t23121 = t6597 * t6589;
    let t23122 = t23121 * t281;
    let t23132 = t6619 * t835;
    let t23133 = t812 * t23132;
    let t23138 = t9223 * t1891;
    let t23139 = t23138 * t213;
    let t23140 = t23139 * t1895;
    let t23141 = F::cast_from(0.11304371706359309439e-1_f64) * t23140;
    let t23143 = t22715 * t206 * t1887;
    (t23121, t23122, t23132, t23133, t23138, t23141, t23143)
}
