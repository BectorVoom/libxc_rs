//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2690/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2690<F: Float>(t20672: F, t225: F, t1372: F, t1386: F, t16022: F, t16460: F, t1843: F, t20044: F, t20060: F, t20594: F, t20609: F, t20613: F, t3758: F, t3882: F, t5210: F, t5326: F, t562: F, t56434: F, t56580: F, t568: F, t6434: F, t6440: F, t6461: F, t74767: F) -> F {
    let t74908 = t20672 * t225;
    let t74929 = t1372 * t20594 * t568 + F::new(3.0) * t5210 * t568 * t6434 + t562 * t568 * t74767 - F::new(3.0) * t1386 * t74908 + F::new(6.0) * t16022 * t6440 + F::new(6.0) * t16460 * t6440 - F::new(3.0) * t16460 * t6461 - F::new(3.0) * t1843 * t56434 - F::new(3.0) * t1843 * t56580 + F::new(6.0) * t20044 * t5326 + F::new(6.0) * t20060 * t5326 - F::new(6.0) * t20609 * t3758 + F::new(6.0) * t20613 * t3882;
    t74929
}
