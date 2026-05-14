//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1235/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1235<F: Float>(t6787: F, t82573: F, t23384: F, t23687: F, t23658: F, t23665: F, t23494: F, t6743: F, t23547: F, t11023: F, t11027: F, t23346: F, t23601: F, t23610: F, t23614: F, t23620: F, t23650: F, t23670: F, t23677: F, t23678: F, t6687: F, t6784: F, t6797: F, t6799: F, t6800: F, t6801: F, t82402: F, t82562: F, t82564: F, t82566: F, t884: F, t986: F) -> (F,) {
    let t82574 = t82573 * t6787;
    let t82576 = t23384 * t23687;
    let t82590 = t23665 * t23658;
    let t82592 = t23494 * t6743;
    let t82596 = t23547 * t6743;
    let t82603 = 0.27415567780803773942e-2 * t82562 + 0.36554090374405031922e-2 * t82564 + 0.82246703342411321826e-2 * t6687 * t6784 * t82566 * t884 - 0.13159472534785811492e0 * t23670 * t23610 - 0.14621636149762012769e-1 * t82574 + 0.54831135561607547883e-2 * t82576 + 0.65797362673929057459e-1 * t23346 * t23650 + 0.82246703342411321825e-2 * t6797 * t6799 * t11027 * t6800 + 0.49348022005446793095e-1 * t23601 * t23677 * t11023 * t23678 + 0.43864908449286038307e-1 * t82402 * t23614 - 0.16449340668482264365e-1 * t82590 - 0.24674011002723396548e-1 * t6797 * t82592 * t6801 - 0.24674011002723396548e-1 * t6797 * t82596 * t6801 - 0.24674011002723396548e-1 * t6687 * t986 * t23620;
    (t82603,)
}
