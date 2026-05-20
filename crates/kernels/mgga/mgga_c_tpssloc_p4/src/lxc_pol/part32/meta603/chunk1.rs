//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1995/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1995<F: Float>(t22706: F, t81046: F, t22863: F, t6979: F, t22641: F, t3749: F, t6978: F, t80854: F, t1984: F, t80845: F, t2010: F, t6973: F, t80742: F) -> (F, F, F, F, F, F, F) {
    let t81047 = t81046 * t22706;
    let t81061 = t22863 * t6979;
    let t81064 = t22641 * t3749;
    let t81066 = t81064 * t80854 * t6978;
    let t81071 = t80845 * t1984;
    let t81072 = t81071 * t2010;
    let t81073 = F::cast_from(0.27720185200590482541e0_f64) * t81072;
    let t81074 = t80742 * t6973;
    (t81047, t81061, t81064, t81066, t81071, t81073, t81074)
}
