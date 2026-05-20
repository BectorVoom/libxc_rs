//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2237/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2237<F: Float>(t13961: F, t4641: F, t14137: F, t4644: F, t1041: F, t13969: F, t17971: F, t17713: F, t3130: F, t17997: F, t3070: F, t42488: F) -> (F, F, F, F, F) {
    let t61794 = t4641 * t13961;
    let t61796 = t4644 * t14137;
    let t61853 = t1041 * t13969 * t17971;
    let t61866 = t3130 * t13969 * t17713;
    let t61916 = t3070 * t42488 * t17997;
    (t61794, t61796, t61853, t61866, t61916)
}
