//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 636/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk636<F: Float>(t7375: F, t8073: F, t1751: F, t2147: F, t462: F, t1734: F, t2144: F, t1246: F, t493: F, t8054: F, t1244: F, t1729: F, t2121: F, t2149: F, t2152: F, t470: F, t7283: F, t7361: F, t7373: F, t7999: F, t8067: F, t8070: F) -> (F, F, F, F, F, F) {
    let t8074 = t7375 * t8073;
    let t8077 = t2147 * t1751;
    let t8078 = t462 * t8077;
    let t8082 = t2144 * t1734;
    let t8083 = t8082 * t1246;
    let t8085 = t493 * t8054;
    let t8087 = -F::new(0.21932454224643019153e-1) * t7999 * t2149 + t7361 - F::new(0.27415567780803773942e-2) * t7283 * t8067 - F::new(0.82246703342411321825e-2) * t7283 * t8070 + F::new(0.82246703342411321825e-2) * t7373 * t8074 + F::new(0.82246703342411321825e-2) * t2121 * t8078 + t1729 * t2152 + t1244 * t8083 + t470 * t8085;
    (t8074, t8077, t8078, t8083, t8085, t8087)
}
