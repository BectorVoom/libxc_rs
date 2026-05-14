//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1268/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1268<F: Float>(t5605: F, t8499: F, t8997: F, t2738: F, t956: F, t983: F, t18069: F, t8984: F, t18067: F, t8507: F, t2782: F, t18126: F, t962: F, t2650: F, t5614: F, t1723: F, t9036: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61411 = t5605 * t8499;
    let t61417 = t5605 * t8997;
    let t61422 = t983 * sigma0 * t956 * t2738;
    let t61425 = t18069 * t8984;
    let t61431 = t18067 * t8507;
    let t61432 = t2782 * t61431;
    let t61437 = t18126 * t962;
    let t61439 = t5614 * t2650;
    let t61442 = t1723 * t9036 / 5184.0;
    (t61411, t61417, t61422, t61425, t61431, t61432, t61437, t61439, t61442)
}
