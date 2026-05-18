//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1288/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1288<F: Float>(t114891: F, t23168: F, t31367: F, t114790: F, t23164: F, t6555: F, t2047: F, t212: F, t23171: F, t6554: F, t31420: F, t6547: F) -> (F, F, F, F, F) {
    let t114892 = F::new(0.26044789391763585244e-1) * t114891;
    let t114900 = t23168 * t31367;
    let t114916 = t23164 * t114790 * t6555;
    let t114932 = t23171 * t212 * t2047 * t6554;
    let t114933 = F::new(0.82246703342411321824e-2) * t114932;
    let t114939 = t6547 * t31420;
    (t114892, t114900, t114916, t114933, t114939)
}
