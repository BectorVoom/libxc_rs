//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1264/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1264<F: Float>(t1888: F, t23270: F, t2717: F, t2742: F, t865: F, t22986: F, t22996: F, t22997: F, t9627: F, t252: F, t2553: F, t6646: F, t829: F) -> (F, F, F) {
    let t81559 = t1888 * t23270 * t2717 * t2742 * t865;
    let t81563 = t22986 * t22996 * t22997 * t9627;
    let t81568 = t22986 * t6646 * t252 * t2553 * t829;
    (t81559, t81563, t81568)
}
