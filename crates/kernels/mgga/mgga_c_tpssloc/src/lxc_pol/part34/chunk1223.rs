//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1223/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1223<F: Float>(t101708: F, t105621: F, t105629: F, t105634: F, t105638: F, t105642: F, t105646: F, t1510: F, t2051: F, t20870: F, t20937: F, t26661: F, t29000: F, t29052: F, t4166: F, t4291: F, t5585: F, t5617: F, t7101: F, t812: F, t87177: F, t92521: F, t98490: F, t98505: F, t98516: F, t98549: F) -> F {
    let t108218 = -F::new(0.23029076935875170111e0) * t98490 + t20937 * t2051 + F::new(0.23029076935875170111e0) * t98505 + F::new(6.0) * t4166 * t29000 + F::new(6.0) * t812 * t92521 * t5585 - F::new(0.14804406601634037928e0) * t98516 - F::new(0.16449340668482264365e-1) * t105621 + F::new(0.49348022005446793095e-1) * t87177 - F::new(0.9869604401089358619e-1) * t105629 - F::new(3.0) * t4291 * t101708 * t1510 - F::new(3.0) * t812 * t26661 * t5617 + F::new(0.49348022005446793095e-1) * t98549 + F::new(0.9869604401089358619e-1) * t105634 - F::new(0.16449340668482264365e-1) * t105638 - F::new(0.49348022005446793095e-1) * t105642 + F::new(0.29608813203268075857e0) * t105646 - F::new(3.0) * t4166 * t29052 - t812 * t7101 * t20870;
    t108218
}
