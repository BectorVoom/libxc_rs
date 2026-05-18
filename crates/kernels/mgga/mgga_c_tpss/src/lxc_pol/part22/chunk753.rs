//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 753/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk753<F: Float>(t2724: F, t948: F, t3932: F, t3931: F, t1465: F, t2675: F, t242: F, t946: F, t837: F, t2741: F, t1461: F, t1467: F, t2665: F, t2670: F, t2682: F, t2685: F, t2690: F, t2722: F, t2740: F, t3917: F, t3920: F, t3924: F, t3928: F, t925: F) -> (F, F, F, F, F, F, F) {
    let t3933 = t2724 * t948;
    let t3934 = t3932 * t3933;
    let t3935 = t3931 * t3934;
    let t3940 = t2675 * t1465;
    let t3941 = t242 * t3940;
    let t3942 = t946 * t3941;
    let t3944 = t1465 * t837;
    let t3945 = t2741 * t3944;
    let t3948 = -t2665 / F::new(108.0) - t2670 + t2690 / F::new(864.0) - t2685 * t1461 / F::new(108.0) + t3917 / F::new(864.0) + t925 * t3920 / F::new(216.0) - t925 * t3924 / F::new(144.0) + t925 * t3928 / F::new(288.0) + t2722 * t3935 / F::new(1536.0) - t2682 * t1467 / F::new(576.0) + t3942 / F::new(4608.0) + t2740 * t3945 / F::new(4608.0);
    (t3933, t3934, t3935, t3941, t3944, t3945, t3948)
}
