//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2324/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2324<F: Float>(t26257: F, t3872: F, t1831: F, t80869: F, t22783: F, t5314: F, t26297: F, t80853: F, t80855: F, t26301: F, t22788: F, t16333: F, t6952: F) -> (F, F, F, F, F, F, F) {
    let t91133 = t26257 * t3872;
    let t91135 = t80869 * t1831;
    let t91136 = F::new(7.0) / F::new(288.0) * t91135;
    let t91137 = t22783 * t5314;
    let t91138 = F::new(7.0) / F::new(288.0) * t91137;
    let t91140 = t80853 * t80855 * t26297;
    let t91141 = F::cast_from(0.40372756094140390854e-3_f64) * t91140;
    let t91143 = t80853 * t80855 * t26301;
    let t91144 = F::cast_from(0.40372756094140390854e-3_f64) * t91143;
    let t91145 = t22788 * t5314;
    let t91147 = t6952 * t16333;
    (t91133, t91136, t91138, t91141, t91144, t91145, t91147)
}
