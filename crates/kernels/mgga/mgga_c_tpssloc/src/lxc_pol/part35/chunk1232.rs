//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1232/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1232<F: Float>(t1888: F, t232: F, t6646: F, t67392: F, t67350: F, t82018: F, t9975: F, t22996: F, t2632: F, t67405: F, t25038: F, t25248: F, t25249: F, t5544: F, t105419: F, t1510: F, t16673: F, t1909: F, t20937: F, t20986: F, t226: F, t235: F, t25261: F, t4281: F, t4291: F, t5612: F, t7533: F, t812: F, t87177: F, t98488: F, t98490: F, t98505: F, t98516: F, t98549: F, t98592: F) -> (F,) {
    let t105621 = t1888 * t6646 * t67392 * t232;
    let t105629 = t1888 * t82018 * t67350 * t9975;
    let t105634 = t1888 * t22996 * t67350 * t2632;
    let t105638 = t1888 * t6646 * t67350 * t232;
    let t105642 = t1888 * t6646 * t67405 * t232;
    let t105646 = t25038 * t25248 * t25249 * t5544;
    let t105650 = t20937 * t1909 + 0.57572692339687925277e-1 * t98488 - 0.11514538467937585055e0 * t98490 - 3.0 * t4291 * t25261 * t5612 + 0.11514538467937585055e0 * t98505 - 3.0 * t812 * t98592 * t1510 - 3.0 * t16673 * t7533 - 0.74022033008170189643e-1 * t98516 - 0.82246703342411321825e-2 * t105621 + 0.24674011002723396547e-1 * t87177 + 6.0 * t4281 * t25261 * t20986 - 0.49348022005446793095e-1 * t105629 + 0.24674011002723396547e-1 * t98549 + 0.49348022005446793095e-1 * t105634 - 0.82246703342411321825e-2 * t105638 - 0.24674011002723396548e-1 * t105642 + 0.14804406601634037928e0 * t105646 + t226 * t235 * t105419;
    (t105650,)
}
