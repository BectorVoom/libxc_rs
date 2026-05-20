//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1294/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1294<F: Float>(t120044: F, t120063: F, t120067: F, t120069: F, t120075: F, t120078: F, t120083: F, t120085: F, t123155: F, t123164: F, t123168: F, t123173: F, t125818: F, t125915: F, t1849: F, t2114: F, t27858: F, t31055: F, t31057: F, t31060: F, t32623: F, t574: F) -> F {
    let t125919 = t120044 - F::new(4.0) * t123155 - t31055 - t31057 - t31060 - F::new(4.0) * t123164 - t120063 - t120067 - t120069 - t120075 - F::new(2.0) * t2114 * t27858 + t120078 + t32623 * t1849 - F::new(4.0) * t123168 - t120083 + (t125818 + t125915) * t574 + t120085 + F::new(6.0) * t123173;
    t125919
}
