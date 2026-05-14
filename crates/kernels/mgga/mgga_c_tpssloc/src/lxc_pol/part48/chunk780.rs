//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 780/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk780<F: Float>(t1378: F, t31641: F, t31611: F, t6891: F, t6888: F, t6883: F, t8622: F, t22666: F, t8621: F, t1985: F, t225: F, t8618: F, t26989: F, t6962: F, t8612: F, t1386: F, t2016: F, t24082: F, t26224: F, t31147: F, t3758: F, t3882: F, t8627: F, t8637: F) -> (F, F, F, F, F, F) {
    let t31642 = t1378 * t31641;
    let t31645 = t31611 * t6891;
    let t31646 = t6888 * t31645;
    let t31648 = t6883 * t8622;
    let t31649 = 0.19190897446562641759e-1 * t31648;
    let t31650 = t22666 * t8621;
    let t31651 = t1985 * t31650;
    let t31653 = t8618 * t225;
    let t31655 = t26989 * t6962;
    let t31662 = t6883 * t8612;
    let t31663 = 0.19190897446562641759e-1 * t31662;
    let t31666 = -0.16449340668482264365e-1 * t31646 - t31147 + t31649 - 0.82246703342411321825e-2 * t31651 - t31653 * t1386 - 6.0 * t26224 * t31655 + 2.0 * t3758 * t8627 + 2.0 * t3882 * t8627 - t31663 - t24082 * t2016 - t3758 * t8637;
    (t31642, t31645, t31650, t31653, t31655, t31666)
}
