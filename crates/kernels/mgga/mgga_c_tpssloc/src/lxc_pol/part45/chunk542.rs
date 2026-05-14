//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 542/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk542<F: Float>(t1055: F, t6815: F, t1052: F, t1066: F, t1920: F, t1923: F, t1956: F, t3026: F, t3169: F, t388: F, t6680: F, t6685: F, t6687: F, t6692: F, t6695: F, t6700: F, t6707: F, t6710: F, t6769: F, t6771: F, t6776: F) -> (F, F) {
    let t6816 = t1055 * t6815;
    let t6818 = -0.21932454224643019153e-1 * t6680 * t1923 + t6685 + 0.27415567780803773942e-2 * t6687 * t6692 - 0.82246703342411321825e-2 * t6687 * t6695 + 0.82246703342411321825e-2 * t1920 * t6700 - 0.82246703342411321825e-2 * t6687 * t6707 + t6710 * t388 + t6769 * t388 - t6771 * t1066 - t3026 * t1956 - t3169 * t1956 + 2.0 * t1052 * t6776 - t1052 * t6816;
    (t6816, t6818)
}
