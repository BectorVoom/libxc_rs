//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1025/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1025<F: Float>(t70582: F, t2211: F, t41122: F, t884: F, t40940: F, t739: F, t77871: F, t72164: F, t72173: F, t14581: F, t8537: F, t14585: F, t8672: F) -> (F, F, F, F, F, F, F, F) {
    let t78582 = F::new(0.86737941314158990619e-4) * t70582;
    let t78585 = F::new(0.11974241701863808564e0) * t884 * t2211 * t41122;
    let t78588 = F::new(0.11974241701863808564e0) * t884 * t2211 * t40940;
    let t78590 = t739 * t77871;
    let t78591 = F::new(0.14967802127329760705e-1) * t78590;
    let t78592 = F::new(0.36021158228745895953e-3) * t72164;
    let t78593 = F::new(0.51240438831339423711e-4) * t72173;
    let t78594 = t14581 * t8537;
    let t78595 = F::new(0.27274661654245341728e-1) * t78594;
    let t78596 = t14585 * t8672;
    (t78582, t78585, t78588, t78591, t78592, t78593, t78595, t78596)
}
