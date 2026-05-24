//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1121/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1121<F: Float>(t76545: F, t15887: F, t302: F, t72: F, t72170: F, t72178: F, t72193: F, t73624: F, t78591: F, t78592: F, t78593: F, t78595: F, t78597: F, t78602: F, t78605: F, t78609: F, t78611: F, t78612: F, t78613: F) -> F {
    let t80537 = F::cast_from(0.40992351065071538966e-4_f64) * t76545;
    let t80538 = t15887 * t302 * t72 - t72170 + t72178 + t72193 - t73624 + t78591 + t78592 - t78593 + t78595 - t78597 - t78602 - t78605 + t78609 - t78611 + t78612 + t78613 - t80537;
    t80538
}
