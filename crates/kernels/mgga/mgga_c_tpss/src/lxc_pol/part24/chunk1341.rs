//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1341/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1341<F: Float>(t1338: F, t547: F, t66108: F, t1281: F, t16067: F, t16073: F, t1784: F, t21546: F, t5474: F, t5477: F, t548: F, t5766: F, t71022: F, t71032: F, t71037: F, t71041: F, t71043: F, t71045: F, t71049: F, t71057: F, t71059: F, t71063: F, t71067: F) -> (F,) {
    let t71070 = 12.0 * t547 * t66108 * t1338;
    let t71071 = t548 * t71022 * param_d + 3.0 * t1281 * t21546 + 12.0 * t16067 * t1784 + 6.0 * t16073 * t1784 + 6.0 * t5474 * t5766 + 3.0 * t5477 * t5766 + t71032 + t71037 + t71041 + t71043 + t71045 + t71049 + t71057 + t71059 + t71063 + t71067 + t71070;
    (t71071,)
}
