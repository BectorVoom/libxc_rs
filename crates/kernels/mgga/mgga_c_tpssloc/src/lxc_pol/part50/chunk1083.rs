//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1083/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1083<F: Float>(t1058: F, t1060: F, t113491: F, t113562: F, t113576: F, t1599: F, t1610: F, t1615: F, t23633: F, t25549: F, t25554: F, t30843: F, t30889: F, t30897: F, t3200: F, t32939: F, t32943: F, t4542: F, t4615: F, t4649: F, t4684: F, t6680: F, t6687: F, t6743: F, t7619: F, t8391: F, t8400: F, t8404: F) -> (F,) {
    let t119393 = t4615 * t8404 + t1610 * t30897 - t3200 * t32943 * t4684 - t113562 - 0.43864908449286038307e-1 * t6680 * t32939 + t1058 * t8391 * t4649 * t1060 + t1058 * t30843 * t1615 * t1060 + 0.54831135561607547883e-2 * t23633 * t113491 * t25549 - 0.14621636149762012769e-1 * t113576 - 0.16449340668482264365e-1 * t6687 * t4542 * t8400 - 0.16449340668482264365e-1 * t6687 * t1599 * t30889 + 0.54831135561607547883e-2 * t23633 * t6743 * t7619 * t25554;
    (t119393,)
}
