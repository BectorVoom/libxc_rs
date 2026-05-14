//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1325/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1325<F: Float>(t1692: F, t1713: F, t17929: F, t19670: F, t19802: F, t20018: F, t20054: F, t21345: F, t21485: F, t21495: F, t21499: F, t2439: F, t3552: F, t5586: F, t5590: F, t5671: F, t64284: F, t70839: F, t70844: F, t70847: F, t70850: F, t70854: F, t70857: F, t70861: F, t70868: F, t70872: F) -> (F,) {
    let t70885 = 3.0 * t3552 * t5586 * t21485 + 3.0 / 2.0 * t2439 * t1713 * t70839 + 6.0 * t19670 * t70844 - 3.0 * t17929 * t70847 - t1692 * t5590 * t70850 / 2.0 + 6.0 * t17929 * t70854 + 3.0 / 2.0 * t2439 * t1713 * t70857 - t1692 * t5590 * t70861 / 2.0 + 3.0 / 2.0 * t2439 * t21345 * t5671 + 3.0 * t2439 * t1713 * t70868 - 3.0 * t17929 * t70872 + 3.0 / 2.0 * t2439 * t5586 * t21499 - 3.0 * t64284 * t20018 - t1692 * t19802 * t20054 + 3.0 * t2439 * t5586 * t21495;
    (t70885,)
}
