//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 880/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk880<F: Float>(t1629: F, t4684: F, t1022: F, t1625: F, t1060: F, t383: F, t4657: F, t1003: F, t1058: F, t1061: F, t1063: F, t1610: F, t1630: F, t1632: F, t3180: F, t3186: F, t3200: F, t353: F, t384: F, t4615: F, t4669: F, t4674: F, t4678: F, t4681: F) -> (F, F, F, F, F) {
    let t4685 = t1629 * t4684;
    let t4688 = t1625 * t1022;
    let t4689 = t4688 * t1060;
    let t4691 = t383 * t4657;
    let t4693 = t1003 * t1632 + t1058 * t4678 + t1058 * t4681 + t1058 * t4689 + t1061 * t4669 + t1063 * t1610 + t1630 * t3180 + F::new(2.0) * t3186 * t4674 - t3200 * t4685 + t353 * t4691 + t384 * t4615;
    (t4685, t4688, t4689, t4691, t4693)
}
