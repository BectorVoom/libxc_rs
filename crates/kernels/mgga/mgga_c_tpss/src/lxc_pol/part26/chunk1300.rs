//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1300/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1300<F: Float>(t5380: F, t5731: F, t1232: F, t1265: F, t13850: F, t1656: F, t1657: F, t1768: F, t18483: F, t18490: F, t18496: F, t19497: F, t19509: F, t19541: F, t19542: F, t19548: F, t19567: F, t21052: F, t21061: F, t21069: F, t21083: F, t21088: F, t21098: F, t21101: F, t21104: F, t4459: F, t4516: F, t520: F, t538: F, t5737: F, t5739: F, t5740: F, t5745: F, t5751: F, t60811: F, t6255: F, t6260: F, t6262: F, t6263: F, t65667: F, t65747: F, t69569: F) -> (F, F) {
    let t69587 = t5731 * t5380;
    let t69631 = t5739 * t5745 * t1768 * t13850 * t520 + t5739 * t5745 * t21052 * t1232 * t520 - 2.0 * t65747 * t1657 + t18483 * t21101 + t5739 * t5745 * t69587 * t520 + 2.0 * t19509 * t19548 + 4.0 * t5739 * t5740 * t19497 * t1656 + param_beta * t69569 * t538 + 4.0 * t5739 * t5740 * t6255 * t4516 + 8.0 * t18496 * t19541 * t1656 * t19542 - 2.0 * t18483 * t21088 + t18483 * t21098 - 12.0 * t5739 * t18490 * t6262 * t4516 - t21061 * t5751 + 4.0 * t65667 * t6263 + 24.0 * t5739 * t60811 * t21069 * t1265 - t5737 * t21104 - 2.0 * t6260 * t19567 + 2.0 * t5739 * t5745 * t6255 * t4459 * t520 + 2.0 * t18483 * t21083;
    (t69587, t69631)
}
