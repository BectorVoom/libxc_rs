//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1311/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1311<F: Float>(t10514: F, t10662: F, t10667: F, t1398: F, t1692: F, t1713: F, t18047: F, t18052: F, t19802: F, t19809: F, t19818: F, t2133: F, t2439: F, t3552: F, t35525: F, t35530: F, t3683: F, t3724: F, t44329: F, t44350: F, t44474: F, t5586: F, t5590: F, t60996: F, t61264: F, t6149: F, t6192: F, t63844: F, t63863: F) -> (F,) {
    let t64855 = -6.0 * t10514 * t19802 * t2439 + 12.0 * t10662 * t1713 * t3552 + 6.0 * t10667 * t1713 * t3552 - t1398 * t1692 * t61264 - 2.0 * t1692 * t18047 * t3724 + 4.0 * t1692 * t18052 * t44350 + 2.0 * t1692 * t18052 * t63844 + 4.0 * t1692 * t19818 * t60996 - 6.0 * t18047 * t19809 * t2439 + 6.0 * t18052 * t2439 * t44474 + 3.0 * t2133 * t2439 * t6149 - 3.0 * t2439 * t35525 * t5590 - 6.0 * t2439 * t44329 * t5590 + 12.0 * t3552 * t3683 * t5586 - 6.0 * t3552 * t5590 * t63863 + 6.0 * t35530 * t6192;
    (t64855,)
}
