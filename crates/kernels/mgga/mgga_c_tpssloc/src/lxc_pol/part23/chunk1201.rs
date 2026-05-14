//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1201/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1201<F: Float>(t13251: F, t1510: F, t16836: F, t16839: F, t20756: F, t20852: F, t20882: F, t20891: F, t20983: F, t232: F, t2632: F, t2643: F, t2645: F, t41467: F, t4178: F, t4180: F, t4181: F, t5544: F, t5587: F, t5593: F, t58574: F, t58576: F, t58642: F, t58811: F, t67620: F, t67852: F, t67854: F) -> (F,) {
    let t76227 = t58811 * t5587 / 256.0 - t4178 * t2645 * t16839 * t2632 * t5544 / 64.0 + t4178 * t4180 * t4181 * t2632 * t20852 / 384.0 - t16836 * t20983 / 32.0 + t58642 * t5593 / 64.0 + 595.0 / 576.0 * t58574 - 119.0 / 1152.0 * t58576 - t2643 * t4180 * t67620 * t1510 / 768.0 + 5.0 / 32.0 * t2643 * t41467 * t4181 * t232 * t20756 + t13251 * t20882 / 64.0 - t13251 * t20891 / 256.0 + 7.0 / 384.0 * t67852 + 7.0 / 384.0 * t67854;
    (t76227,)
}
