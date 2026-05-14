//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1029/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1029<F: Float>(t140: F, t928: F, t3754: F, t925: F, t2697: F, t3749: F, t11018: F, t3919: F, t11022: F, t8491: F, t926: F, t11008: F, t11031: F, t3923: F, t11035: F, t242: F, t2751: F, t3758: F) -> (F, F, F, F, F, F, F, F) {
    let t11521 = t140 * t928;
    let t11522 = t11521 * t3754;
    let t11524 = t925 * t11522 / 216.0;
    let t11525 = t140 * t2697;
    let t11526 = t11525 * t3749;
    let t11528 = t925 * t11526 / 324.0;
    let t11529 = t3919 * t11018;
    let t11532 = t3919 * t11022;
    let t11535 = t926 * t8491;
    let t11536 = t11535 * t11008;
    let t11539 = t3923 * t11031;
    let t11542 = t3923 * t11035;
    let t11548 = t242 * t2751 * t3758;
    (t11524, t11528, t11529, t11532, t11536, t11539, t11542, t11548)
}
