//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1329/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1329<F: Float>(t13220: F, t1338: F, t19261: F, t20706: F, t20957: F, t2105: F, t3537: F, t5986: F, t63626: F, t645: F, t65993: F, t65995: F, t65997: F, t65999: F, t68151: F, t68152: F, t68156: F, t68163: F, t68168: F) -> (F,) {
    let t68173 = 2.0 * t13220 * t5986 + 2.0 * t1338 * t63626 + 4.0 * t1338 * t68163 + 2.0 * t1338 * t68168 + 4.0 * t19261 * t3537 + 4.0 * t20706 * t3537 + 2.0 * t20957 * t2105 + 4.0 * t645 * t68156 + t65993 + t65995 + t65997 + t65999 + t68151 + 2.0 * t68152;
    (t68173,)
}
