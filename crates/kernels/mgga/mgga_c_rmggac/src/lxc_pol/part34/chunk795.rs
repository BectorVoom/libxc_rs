//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 795/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk795<F: Float>(t76732: F, t15502: F, t3351: F, t352: F, t7231: F, t875: F, t118: F, t2001: F, t618: F, t699: F, t7720: F, t73851: F, t73854: F, t73865: F, t73871: F, t73873: F) -> (F, F, F, F, F, F, F, F) {
    let t76733 = 0.12769379967989351819e-4 * t76732;
    let t76737 = t3351 * t7231 * t875 * t15502 * t352;
    let t76738 = 0.85129199786595678796e-5 * t76737;
    let t76741 = t2001 * t118 * t699 * t618;
    let t76742 = t7720 * t76741;
    let t76743 = 0.42564599893297839398e-5 * t76742;
    let t76744 = 0.2627895913935205078e-5 * t73851;
    let t76745 = 0.2627895913935205078e-5 * t73854;
    let t76748 = 0.19709219354514038085e-5 * t73865;
    let t76749 = 0.64054962902170623776e-5 * t73871;
    let t76750 = 0.85129199786595678799e-5 * t73873;
    (t76733, t76738, t76743, t76744, t76745, t76748, t76749, t76750)
}
