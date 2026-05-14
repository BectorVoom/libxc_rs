//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1037/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1037<F: Float>(t3791: F, t562: F, t550: F, t6976: F, t1992: F, t6914: F, t6979: F, t3734: F, t6968: F, t6637: F, t22685: F, t6546: F, t6887: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22740 = t562 * t3791;
    let t22741 = t22740 * t550;
    let t22742 = t6976 * t22741;
    let t22743 = t1992 * t22742;
    let t22745 = t6914 * t6979;
    let t22746 = 0.38381794893125283518e-1 * t22745;
    let t22747 = t6968 * t3734;
    let t22748 = t6637 * t22747;
    let t22749 = t22685 * t22748;
    let t22751 = t6546 * t6887;
    (t22740, t22741, t22742, t22743, t22746, t22747, t22748, t22749, t22751)
}
