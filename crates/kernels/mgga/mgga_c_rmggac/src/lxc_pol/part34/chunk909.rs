//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 909/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk909<F: Float>(t739: F, t77871: F, t72164: F, t72173: F, t14581: F, t8537: F, t14585: F, t8672: F, t1356: F, t13957: F, t43974: F, t7879: F, t884: F, t9530: F, t577: F, t703: F, t7933: F, t7934: F) -> (F, F, F, F, F, F, F, F) {
    let t78590 = t739 * t77871;
    let t78591 = 0.14967802127329760705e-1 * t78590;
    let t78592 = 0.36021158228745895953e-3 * t72164;
    let t78593 = 0.51240438831339423711e-4 * t72173;
    let t78594 = t14581 * t8537;
    let t78595 = 0.27274661654245341728e-1 * t78594;
    let t78596 = t14585 * t8672;
    let t78597 = 0.36366215538993788971e-1 * t78596;
    let t78602 = 0.11974241701863808564e0 * t1356 * t43974 * t13957;
    let t78605 = 0.11974241701863808564e0 * t884 * t9530 * t7879;
    let t78608 = t7933 * t7934 * t577 * t703;
    (t78591, t78592, t78593, t78595, t78597, t78602, t78605, t78608)
}
