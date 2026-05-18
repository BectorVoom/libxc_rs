//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1331/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1331<F: Float>(t105419: F, t105621: F, t105629: F, t105634: F, t105638: F, t105642: F, t105646: F, t1510: F, t16673: F, t1909: F, t20937: F, t20986: F, t226: F, t235: F, t25261: F, t4281: F, t4291: F, t5612: F, t7533: F, t812: F, t87177: F, t98488: F, t98490: F, t98505: F, t98516: F, t98549: F, t98592: F) -> F {
    let t105650 = t20937 * t1909 + F::new(0.57572692339687925277e-1) * t98488 - F::new(0.11514538467937585055e0) * t98490 - F::new(3.0) * t4291 * t25261 * t5612 + F::new(0.11514538467937585055e0) * t98505 - F::new(3.0) * t812 * t98592 * t1510 - F::new(3.0) * t16673 * t7533 - F::new(0.74022033008170189643e-1) * t98516 - F::new(0.82246703342411321825e-2) * t105621 + F::new(0.24674011002723396547e-1) * t87177 + F::new(6.0) * t4281 * t25261 * t20986 - F::new(0.49348022005446793095e-1) * t105629 + F::new(0.24674011002723396547e-1) * t98549 + F::new(0.49348022005446793095e-1) * t105634 - F::new(0.82246703342411321825e-2) * t105638 - F::new(0.24674011002723396548e-1) * t105642 + F::new(0.14804406601634037928e0) * t105646 + t226 * t235 * t105419;
    t105650
}
