//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1023/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1023<F: Float>(t76506: F, t70506: F, t70514: F, t70526: F, t15669: F, t2604: F, t8264: F, t884: F, t8946: F, t70549: F, t638: F, t639: F, t702: F, t8849: F) -> (F, F, F, F, F, F, F, F) {
    let t78546 = F::new(0.23948483403727617128e0) * t76506;
    let t78547 = F::new(0.10248087766267884741e-3) * t70506;
    let t78548 = F::new(0.72042316457491791901e-3) * t70514;
    let t78551 = F::new(0.79828278012425390427e-1) * t70526;
    let t78553 = F::new(0.11974241701863808564e0) * t2604 * t15669;
    let t78556 = F::new(0.11974241701863808564e0) * t884 * t8264 * t8946;
    let t78557 = F::new(0.638468998399467591e-4) * t70549;
    let t78560 = t638 * t639 * t8849 * t702;
    (t78546, t78547, t78548, t78551, t78553, t78556, t78557, t78560)
}
