//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2694/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2694<F: Float>(t56475: F, t56525: F, t56542: F, t56568: F, t20032: F, t225: F, t20040: F, t12033: F, t1386: F, t16022: F, t16437: F, t16452: F, t16453: F, t16475: F, t1843: F, t20023: F, t20029: F, t20044: F, t20060: F, t26224: F, t3752: F, t3882: F, t3889: F, t3912: F, t5215: F, t5321: F, t5354: F, t55093: F, t55118: F, t562: F, t568: F, t6434: F, t6440: F, t6461: F) -> (F, F) {
    let t56570 = t56475 + t56525 + t56542 + t56568;
    let t56580 = t20032 * t225;
    let t56596 = t20040 * t225;
    let t56605 = -F::new(24.0) * t16452 * t26224 * t55118 + t3752 * t568 * t6434 + t562 * t56570 * t568 + F::new(2.0) * t12033 * t6440 - t12033 * t6461 - F::new(2.0) * t1386 * t56580 - F::new(2.0) * t1386 * t56596 - F::new(4.0) * t16022 * t5354 - F::new(2.0) * t16437 * t5321 + F::new(8.0) * t16453 * t5215 - F::new(12.0) * t16475 * t5321 - F::new(4.0) * t1843 * t55093 - F::new(2.0) * t20023 * t3882 + F::new(4.0) * t20029 * t3889 + F::new(2.0) * t20044 * t3889 - t20044 * t3912 - t20060 * t3912;
    (t56570, t56605)
}
