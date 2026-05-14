//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 831/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk831<F: Float>(t112760: F, t2649: F, t30714: F, t23109: F, t23110: F, t232: F, t59: F, t828: F, t1894: F, t23078: F, t2379: F, t23062: F, t30700: F, t2553: F, t6591: F, t240: F, t241: F, t2627: F, t812: F) -> (F, F, F, F, F, F, F) {
    let t112761 = 0.76763589786250567036e-1 * t112760;
    let t112773 = t30714 * t2649;
    let t112778 = t23109 * t23110 * t59 * t828 * t232;
    let t112782 = t23078 * t1894 * t59 * t2379;
    let t112784 = t23062 * t30700;
    let t112788 = t6591 * t1894 * t59 * t2553;
    let t112792 = t812 * t2627 * t240 * t241;
    (t112761, t112773, t112778, t112782, t112784, t112788, t112792)
}
