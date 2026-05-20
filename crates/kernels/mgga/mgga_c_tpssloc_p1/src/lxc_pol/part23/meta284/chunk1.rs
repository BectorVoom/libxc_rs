//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 981/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk981<F: Float>(t16839: F, t20981: F, t2645: F, t2632: F, t5611: F, t4180: F, t4181: F, t119: F, t20800: F, t210: F, t13251: F, t16940: F, t20963: F, t20969: F, t20974: F, t20978: F, t2630: F, t2643: F, t4167: F, t4178: F, t5593: F, t5614: F, t5619: F, t787: F, t817: F) -> (F, F, F, F, F, F) {
    let t20983 = t2645 * t16839 * t20981;
    let t20986 = t2632 * t5611;
    let t20988 = t4180 * t4181 * t20986;
    let t20993 = t119 * t20800;
    let t20994 = t210 * t20993;
    let t20998 = -t4167 * t5614 / F::new(1024.0) + t2630 * t20963 / F::new(512.0) - t4167 * t5619 / F::new(1024.0) - t817 * t20969 / F::new(3072.0) - F::new(5.0) / F::new(256.0) * t2643 * t20974 + t2643 * t20978 / F::new(256.0) - t4178 * t20983 / F::new(128.0) + t4178 * t20988 / F::new(512.0) + t13251 * t5593 / F::new(128.0) - t787 * t20994 / F::new(48.0) + F::new(7.0) / F::new(1536.0) * t16940;
    (t20983, t20986, t20988, t20993, t20994, t20998)
}
