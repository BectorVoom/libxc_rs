//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1247/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1247<F: Float>(t2782: F, t61431: F, t2650: F, t5614: F, t1723: F, t9036: F, t2798: F, t18170: F, t5628: F, t1726: F, t8552: F, t1729: F, t5637: F, t9066: F, t23310: F, t18155: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61432 = t2782 * t61431;
    let t61439 = t5614 * t2650;
    let t61442 = t1723 * t9036 / 5184.0;
    let t61449 = t2798 * t61431;
    let t61476 = t5628 * t18170;
    let t61489 = t8552 * t1726;
    let t61522 = t1729 * t5637 * t9066;
    let t61540 = t1729 * t23310;
    let t61567 = t5628 * t18155;
    (t61432, t61439, t61442, t61449, t61476, t61489, t61522, t61540, t61567)
}
