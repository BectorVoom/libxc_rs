//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1048/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1048<F: Float>(t1089: F, t1240: F, t225: F, t3597: F, t2131: F, t23508: F, t7325: F, t3030: F, t3502: F, t478: F, t1209: F, t2141: F, t3540: F, t3: F, t7324: F, t2127: F, t3545: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t24602 = t1240 * t1089;
    let t24615 = t225 * t3597;
    let t24658 = t2131 * t23508;
    let t24659 = t24658 * t7325;
    let t24660 = t3030 * t3502;
    let t24661 = t24660 * t478;
    let t24667 = t3030 * t1209;
    let t24668 = t24667 * t478;
    let t24681 = t2141 * t3540 / 6912.0;
    let t24682 = t7324 * t3;
    let t24704 = t2127 * t3545 / 432.0;
    (t24602, t24615, t24658, t24659, t24660, t24661, t24667, t24668, t24681, t24682, t24704)
}
