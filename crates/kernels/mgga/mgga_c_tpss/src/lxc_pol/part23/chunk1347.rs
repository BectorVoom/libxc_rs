//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1347/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1347<F: Float>(t10445: F, t13228: F, t13235: F, t1897: F, t19261: F, t2054: F, t2056: F, t2065: F, t20944: F, t20950: F, t20957: F, t20969: F, t3491: F, t3499: F, t3537: F, t3542: F, t4341: F, t5984: F, t5986: F, t6054: F, t624: F, t626: F, t646: F, t6486: F, t6540: F, t66005: F, t66009: F, t66011: F, t66013: F, t66015: F, t66017: F, t68156: F) -> (F,) {
    let t68728 = -4.0 * t626 * t6054 * t3537 + t66005 - 4.0 * t2056 * t20950 - 2.0 * t13235 * t6486 - 4.0 * t3499 * t20969 - 4.0 * t68156 * t646 - 4.0 * t20957 * t2065 - 2.0 * t5986 * t13228 - 4.0 * t19261 * t3542 - 2.0 * t5984 * t4341 - t2054 * t6540 - 2.0 * t624 * t20944 - t10445 * t1897 - 2.0 * t3491 * t6054 - t66009 - t66011 - t66013 - t66015 + t66017;
    (t68728,)
}
