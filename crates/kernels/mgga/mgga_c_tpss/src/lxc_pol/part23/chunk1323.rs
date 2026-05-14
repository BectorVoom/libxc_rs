//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1323/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1323<F: Float>(t1860: F, t65157: F, t65165: F, t1675: F, t18305: F, t18331: F, t18350: F, t19232: F, t19417: F, t20713: F, t20777: F, t5483: F, t5979: F, t62007: F, t62024: F, t6471: F, t6475: F, t65152: F, t65162: F, t65202: F, t68003: F) -> (F,) {
    let t68006 = t1860 * t65157;
    let t68009 = t1860 * t65165;
    let t68019 = 2.0 / 3.0 * t19417 * t5979 - 10.0 * t19232 * t65202 - 10.0 * t19232 * t65162 - 5.0 * t19232 * t65152 - 10.0 / 3.0 * t62007 * t20713 - 5.0 / 3.0 * t62024 * t20713 - 10.0 / 3.0 * t18350 * t68003 - 10.0 / 3.0 * t18350 * t68006 - 10.0 / 3.0 * t18350 * t68009 - t1675 * t6471 * t18331 / 6.0 - t18305 * t6475 / 6.0 - t5483 * t20777 / 3.0;
    (t68019,)
}
