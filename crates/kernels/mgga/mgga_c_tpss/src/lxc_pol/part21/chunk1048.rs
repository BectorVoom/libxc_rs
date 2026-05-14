//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1048/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1048<F: Float>(t8493: F, t8539: F, t11476: F, t3931: F, t10416: F, t3977: F, t10412: F, t3758: F, t949: F, t2741: F, t1465: F, t2469: F, t2460: F, t8523: F, t242: F, t8469: F) -> (F, F, F, F, F, F, F) {
    let t11661 = t8539 * t8493;
    let t11662 = t11661 * t11476;
    let t11663 = t3931 * t11662;
    let t11666 = t3977 * t10416;
    let t11667 = t3931 * t11666;
    let t11670 = t3977 * t10412;
    let t11671 = t3931 * t11670;
    let t11674 = t3758 * t949;
    let t11675 = t2741 * t11674;
    let t11678 = t1465 * t2469;
    let t11679 = t2741 * t11678;
    let t11682 = t1465 * t2460;
    let t11683 = t8523 * t11682;
    let t11687 = t242 * t8469 * t1465;
    (t11663, t11667, t11671, t11675, t11679, t11683, t11687)
}
