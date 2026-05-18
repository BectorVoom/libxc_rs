//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 968/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk968<F: Float>(t1378: F, t20661: F, t20594: F, t562: F, t1834: F, t6361: F, t1375: F, t1843: F, t20029: F, t20044: F, t20060: F, t20420: F, t20602: F, t20609: F, t20613: F, t5215: F, t5321: F, t568: F, t6440: F, t6461: F) -> (F, F, F, F) {
    let t20662 = t1378 * t20661;
    let t20670 = t20594 * t562;
    let t20672 = t6361 * t1834;
    let t20675 = -F::new(6.0) * t1375 * t20609 + F::new(6.0) * t1375 * t20613 - t1375 * t20662 - F::new(6.0) * t1843 * t20029 - F::new(3.0) * t1843 * t20044 - F::new(3.0) * t1843 * t20060 + F::new(3.0) * t20420 * t568 + t20602 * t568 + t20670 * t568 + F::new(3.0) * t20672 * t568 + F::new(6.0) * t5215 * t6440 - F::new(3.0) * t5215 * t6461 + F::new(6.0) * t5321 * t6440 - F::new(3.0) * t5321 * t6461;
    (t20662, t20670, t20672, t20675)
}
