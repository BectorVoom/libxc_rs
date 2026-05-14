//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1253/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1253<F: Float>(t90: F, t29: F, t2435: F, t251: F, t8346: F, t2813: F, t196: F, t8546: F, t73: F, t8549: F, t371: F, t9065: F, t1364: F, t2428: F, t198: F, t2116: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31462 = t90 * t90;
    let t31464 = t29 / t31462;
    let t31813 = t2435 * t2435;
    let t31814 = 1.0 / t31813;
    let t32386 = 1.0 / t8346 / t251;
    let t32518 = t2813 * t2813;
    let t32519 = 1.0 / t32518;
    let t33457 = 1.0 / t8546 / t196;
    let t33459 = t8549 * t73;
    let t33858 = 1.0 / t9065 / t371;
    let t35525 = t1364 * t2428;
    let t35530 = t198 * t2116;
    (t31464, t31814, t32386, t32519, t33457, t33459, t33858, t35525, t35530)
}
