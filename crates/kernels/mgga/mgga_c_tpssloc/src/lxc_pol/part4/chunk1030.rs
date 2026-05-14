//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1030/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1030<F: Float>(t1100: F, t18730: F, t1107: F, t11243: F, t5992: F, t1102: F, t4756: F, t4764: F, t3287: F, t5999: F, t11265: F, t4748: F, t11211: F, t11372: F, t14702: F, t14705: F, t14711: F) -> (F, F, F, F, F, F, F, F) {
    let t18731 = t1100 * t18730;
    let t18742 = t1107 * t18730;
    let t18746 = t11243 * t5992;
    let t18747 = t18746 * t1102;
    let t18749 = t4764 * t4756;
    let t18751 = t3287 * t5999;
    let t18752 = t18751 * t1102;
    let t18754 = t11265 * t5992;
    let t18755 = t18754 * t1102;
    let t18757 = t4748 * t4756;
    let t18759 = 0.16504875e0 * t18742 - t11372 + 0.26837777777777777779e0 * t14702 - t14705 - t14711 + 0.91983333333333333333e-1 * t11211 - 0.412621875e-1 * t18747 + 0.16504875e0 * t18749 + 0.82524375e-1 * t18752 + 0.19419375e1 * t18755 - 0.258925e1 * t18757;
    (t18731, t18742, t18747, t18749, t18752, t18755, t18757, t18759)
}
