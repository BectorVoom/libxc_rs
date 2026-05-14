//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1269/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1269<F: Float>(t2798: F, t61431: F, t18170: F, t5628: F, t1726: F, t8552: F, t18130: F, t940: F, t1729: F, t5637: F, t9066: F, t18139: F, t5570: F, t23310: F, t18131: F, t219: F) -> (F, F, F, F, F, F, F, F) {
    let t61449 = t2798 * t61431;
    let t61476 = t5628 * t18170;
    let t61489 = t8552 * t1726;
    let t61498 = t940 * t18130;
    let t61522 = t1729 * t5637 * t9066;
    let t61537 = t18139 * t5570;
    let t61540 = t1729 * t23310;
    let t61564 = t18131 * t219;
    (t61449, t61476, t61489, t61498, t61522, t61537, t61540, t61564)
}
