//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1240/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1240<F: Float>(t2435: F, t251: F, t8346: F, t196: F, t8546: F, t73: F, t8549: F, t471: F, t9737: F, t3153: F, t1364: F, t2428: F, t198: F, t2116: F, t1378: F, t8279: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31813 = t2435 * t2435;
    let t31814 = 1.0 / t31813;
    let t32386 = 1.0 / t8346 / t251;
    let t33457 = 1.0 / t8546 / t196;
    let t33459 = t8549 * t73;
    let t35167 = 1.0 / t9737 / t471;
    let t35289 = t3153 * t3153;
    let t35290 = 1.0 / t35289;
    let t35525 = t1364 * t2428;
    let t35530 = t198 * t2116;
    let t35764 = t1378 * t8279;
    (t31814, t32386, t33457, t33459, t35167, t35290, t35525, t35530, t35764)
}
