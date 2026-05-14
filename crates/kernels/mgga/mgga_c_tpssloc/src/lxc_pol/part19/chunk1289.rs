//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1289/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1289<F: Float>(t11205: F, t699: F, t11208: F, t11219: F, t136: F, t43792: F, t3297: F, t43796: F, t1113: F, t43800: F, t43804: F, t43759: F, t43766: F, t43768: F, t43770: F, t43773: F, t43777: F, t43833: F, t43835: F) -> (F, F, F, F, F, F, F) {
    let t43837 = t699 * t11205;
    let t43839 = t699 * t11208;
    let t43842 = t136 * t11219 * t43792;
    let t43845 = t136 * t3297 * t43796;
    let t43848 = t136 * t1113 * t43800;
    let t43851 = t136 * t1113 * t43804;
    let t43853 = 0.49671e0 * t43759 - 0.8585111111111111111e-1 * t43766 + 0.22076e0 * t43768 - 0.132456e1 * t43770 + 0.22076e0 * t43773 + t43777 + 0.16504875e0 * t43833 + 0.44152e0 * t43835 - 0.132456e1 * t43837 - 0.22076e0 * t43839 + 0.44152e0 * t43842 - 0.99342e0 * t43845 + 0.198684e1 * t43848 + 0.82785e-1 * t43851;
    (t43837, t43839, t43842, t43845, t43848, t43851, t43853)
}
