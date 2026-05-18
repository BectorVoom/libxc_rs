//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1092/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1092<F: Float>(t71278: F, t73536: F, t75326: F, t75328: F, t75330: F, t78132: F, t78133: F, t78134: F, t78135: F, t78136: F, t78140: F, t78141: F, t78142: F, t78143: F, t78144: F, t78145: F) -> F {
    let t80355 = -t78132 - t73536 + t71278 - t78133 + t78134 - t78135 - t78136 + F::new(0.17357833928249762291e-2) * t75326 + F::new(0.36366215538993788973e-1) * t75328 - F::new(0.72732431077987577947e-1) * t75330 - t78140 + t78141 + t78142 + t78143 - t78144 + t78145;
    t80355
}
