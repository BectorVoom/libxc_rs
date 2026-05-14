//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 959/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk959<F: Float>(t1634: F, t4693: F, t3174: F, t225: F, t5851: F, t17183: F, t977: F, t17178: F, t2979: F, t17161: F, t10214: F, t17152: F, t1040: F, t5904: F, t248: F, t3101: F, t5867: F) -> (F, F, F, F, F, F, F, F) {
    let t17582 = t1634 * t4693;
    let t17583 = t3174 * t17582;
    let t17588 = t5851 * t225;
    let t17593 = t977 * t17183;
    let t17596 = t2979 * t17178;
    let t17599 = t2979 * t17161;
    let t17602 = t10214 * t17152;
    let t17607 = t5904 * t1040;
    let t17611 = t248 * t3101 * t5867;
    (t17583, t17588, t17593, t17596, t17599, t17602, t17607, t17611)
}
