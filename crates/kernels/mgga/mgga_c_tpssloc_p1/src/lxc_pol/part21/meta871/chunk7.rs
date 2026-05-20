//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3207/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3207<F: Float>(t11888: F, t11914: F, t1216: F, t14992: F, t15032: F, t15245: F, t15429: F, t15772: F, t1729: F, t19145: F, t19153: F, t19156: F, t19169: F, t19170: F, t3565: F, t3604: F, t3610: F, t45323: F, t4964: F, t5011: F, t5068: F, t5076: F, t5086: F, t52480: F, t53545: F, t6256: F, t6260: F, t6263: F, t6265: F) -> F {
    let t66769 = F::new(4.0) * t1216 * t5011 * t52480 * t53545 - F::new(12.0) * t11888 * t19145 * t19156 + F::new(2.0) * t11914 * t15429 * t6256 + t11914 * t15429 * t6260 + F::new(2.0) * t11914 * t19145 * t19153 + F::new(8.0) * t19169 * t3610 * t5068 - F::new(4.0) * t14992 * t15245 + F::new(4.0) * t15032 * t5076 + F::new(2.0) * t15772 * t1729 + F::new(4.0) * t19170 * t3604 + t3565 * t6265 - t45323 * t6263 + F::new(4.0) * t4964 * t5086;
    t66769
}
