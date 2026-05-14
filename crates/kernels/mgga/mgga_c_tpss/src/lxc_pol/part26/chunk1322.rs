//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1322/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1322<F: Float>(t1364: F, t19818: F, t8096: F, t19809: F, t44169: F, t14076: F, t14245: F, t14256: F, t1692: F, t1713: F, t17929: F, t19670: F, t19798: F, t198: F, t19802: F, t207: F, t21345: F, t2439: F, t3552: F, t3683: F, t4706: F, t51780: F, t52613: F, t5586: F, t5590: F, t6149: F, t64305: F, t69810: F, t69863: F, t70212: F, t750: F, t823: F) -> (F,) {
    let t70759 = t8096 * t1364 * t19818;
    let t70771 = t44169 * t19809;
    let t70783 = -6.0 * t2439 * t19802 * t14076 - 3.0 * t2439 * t5590 * t52613 + 12.0 * t3552 * t1713 * t14245 - 6.0 * t3552 * t5590 * t51780 - 6.0 * t2439 * t5590 * t69810 + 6.0 * t2439 * t19798 * t1364 + 4.0 * t1692 * t64305 * t19818 - 3.0 * t2439 * t5590 * t69863 + 12.0 * t17929 * t70759 + 3.0 * t2439 * t21345 * t750 + 12.0 * t3552 * t6149 * t3683 + 6.0 * t3552 * t1713 * t14256 - 12.0 * t19670 * t70771 + t198 * t207 * t70212 * t823 - 6.0 * t2439 * t19802 * t19809 + 6.0 * t3552 * t5586 * t4706;
    (t70783,)
}
