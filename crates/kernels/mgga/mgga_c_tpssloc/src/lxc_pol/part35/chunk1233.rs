//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1233/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1233<F: Float>(t2148: F, t6146: F, t6140: F, t2144: F, t6224: F, t3625: F, t6218: F, t1246: F, t27536: F, t8073: F, t1734: F, t8054: F) -> (F, F, F, F, F, F, F) {
    let t29702 = t6146 * t2148;
    let t29705 = t6140 * t2148;
    let t29708 = t2144 * t6224;
    let t29709 = t29708 * t3625;
    let t29711 = t2144 * t6218;
    let t29712 = t29711 * t1246;
    let t29716 = t27536 * t8073;
    let t29719 = t8054 * t1734;
    (t29702, t29705, t29708, t29709, t29712, t29716, t29719)
}
