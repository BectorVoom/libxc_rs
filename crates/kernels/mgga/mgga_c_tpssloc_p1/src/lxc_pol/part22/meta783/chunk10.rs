//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2689/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2689<F: Float>(t1375: F, t1385: F, t16022: F, t16030: F, t16439: F, t1843: F, t20023: F, t20026: F, t20029: F, t20051: F, t20608: F, t20613: F, t20662: F, t3758: F, t3887: F, t40591: F, t5215: F, t5318: F, t5321: F, t5353: F, t5354: F, t56422: F, t568: F, t6361: F, t6440: F, t6460: F, t6461: F) -> F {
    let t74899 = F::new(24.0) * t1375 * t1385 * t20608 * t40591 + F::new(6.0) * t1375 * t3887 * t5353 * t6460 + F::new(3.0) * t5318 * t568 * t6361 - F::new(3.0) * t16022 * t6461 - F::new(3.0) * t16030 * t6461 + F::new(6.0) * t16439 * t6440 - F::new(6.0) * t1843 * t56422 - F::new(3.0) * t20023 * t5321 + F::new(6.0) * t20026 * t5321 - F::new(6.0) * t20029 * t5354 - F::new(18.0) * t20051 * t5215 + F::new(6.0) * t20613 * t3758 - t20662 * t3758;
    t74899
}
