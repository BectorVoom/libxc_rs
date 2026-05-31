//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2688/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2688<F: Float>(t20602: F, t225: F, t20420: F, t1323: F, t1375: F, t1385: F, t1386: F, t16030: F, t16439: F, t1807: F, t1843: F, t20009: F, t20023: F, t20025: F, t20601: F, t20661: F, t20662: F, t26224: F, t3882: F, t3887: F, t5215: F, t539: F, t55118: F, t56596: F, t56607: F, t568: F, t6440: F, t6461: F, t74837: F) -> F {
    let t74849 = t20602 * t225;
    let t74860 = t20420 * t225;
    let t74868 = F::cast_from(2.0_f64) * t1375 * t1385 * t20661 * t3887 + t1323 * t20601 * t568 + F::cast_from(3.0_f64) * t1807 * t20009 * t568 - F::cast_from(18.0_f64) * t20025 * t26224 * t55118 + t539 * t568 * t74837 - t1386 * t74849 - F::cast_from(3.0_f64) * t1386 * t74860 + F::cast_from(6.0_f64) * t16030 * t6440 - F::cast_from(3.0_f64) * t16439 * t6461 - F::cast_from(3.0_f64) * t1843 * t56596 - F::cast_from(6.0_f64) * t1843 * t56607 - F::cast_from(3.0_f64) * t20023 * t5215 - t20662 * t3882;
    t74868
}
