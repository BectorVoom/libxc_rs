//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1954/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1954<F: Float>(t16944: F, t25891: F, t25927: F, t98111: F, t1649: F, t4119: F, t23788: F, t67123: F, t1081: F, t5660: F, t5544: F, t16662: F, t28: F) -> (F, F, F, F, F, F, F) {
    let t100708 = t25891 * t16944;
    let t100713 = t25927 * t98111;
    let t100718 = t1649 * t4119;
    let t100731 = t23788 * t67123;
    let t100734 = t1081 * t5660;
    let t100743 = t1081 * t5544;
    let t100747 = t28 * t16662;
    (t100708, t100713, t100718, t100731, t100734, t100743, t100747)
}
