//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1443/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1443<F: Float>(t1174: F, t3471: F, t698: F, t3475: F, t3469: F, t3477: F, t11504: F, t135: F, t43713: F, t43717: F, t43721: F, t43725: F, t43754: F, t43759: F, t43766: F, t43768: F, t43770: F, t43773: F, t43835: F) -> (F, F, F, F, F, F) {
    let t44424 = t1174 * t698 * t3471;
    let t44426 = t3475 * t3475;
    let t44432 = t3469 * t3469;
    let t44439 = t1174 * t698 * t3477;
    let t44445 = t1174 * t135 * t11504;
    let t44457 = -F::new(4.0) / F::new(9.0) * t43768 + F::new(2.0) * t43713 + t43754 / F::new(6.0) + F::new(2.0) / F::new(9.0) * t43717 - F::new(6.0) * t43721 - t43759 - F::new(4.0) / F::new(3.0) * t43725 + F::new(14.0) / F::new(81.0) * t43766 + F::new(8.0) / F::new(3.0) * t43770 - F::new(4.0) / F::new(9.0) * t43773 - F::new(8.0) / F::new(9.0) * t43835;
    (t44424, t44426, t44432, t44439, t44445, t44457)
}
