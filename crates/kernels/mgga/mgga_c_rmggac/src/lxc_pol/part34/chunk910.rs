//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 910/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk910<F: Float>(t78608: F, t76547: F, t70618: F, t76550: F, t14530: F, t534: F, t72: F, t72170: F, t72178: F, t72192: F, t72193: F, t76545: F, t78591: F, t78592: F, t78593: F, t78595: F, t78597: F, t78602: F, t78605: F) -> (F,) {
    let t78609 = 0.36021158228745895953e-3 * t78608;
    let t78611 = 0.20496175532535769483e-3 * t76547;
    let t78612 = 0.16263363996404810741e-4 * t70618;
    let t78613 = 0.14967802127329760705e-1 * t76550;
    let t78614 = t78591 + t78592 - t72170 - t78593 + t72178 + t78595 - t78597 + t72 * t534 * t14530 - t78602 - t78605 + t78609 - 0.40992351065071538964e-4 * t76545 - t78611 - t72192 + t72193 + t78612 + t78613;
    (t78614,)
}
