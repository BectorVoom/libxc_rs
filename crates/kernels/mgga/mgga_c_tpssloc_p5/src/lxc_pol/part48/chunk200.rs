//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 200/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk200<F: Float>(t240: F, t815: F, t812: F, t241: F, t244: F, t67: F, t120: F, t246: F) -> (F, F, F, F, F) {
    let t816 = t815 * t240;
    let t817 = t812 * t816;
    let t818 = t241 * t244;
    let t819 = t818 * t67;
    let t820 = t246 * t120;
    (t816, t817, t818, t819, t820)
}
