//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 967/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk967<F: Float>(t1880: F, t24281: F, t6553: F, t6571: F, t31420: F, t6547: F, t23171: F, t23228: F, t8547: F, t31370: F, t114866: F, t6572: F) -> (F, F, F, F, F) {
    let t114937 = t1880 * t6553 * t6571 * t24281;
    let t114939 = t6547 * t31420;
    let t114943 = t23171 * t23228 * t8547;
    let t114944 = F::new(0.82246703342411321824e-2) * t114943;
    let t114945 = t6547 * t31370;
    let t114960 = t1880 * t114866 * t6572;
    (t114937, t114939, t114944, t114945, t114960)
}
