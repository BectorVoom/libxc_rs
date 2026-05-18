//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1203/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1203<F: Float>(t11613: F, t11925: F, t11928: F, t1252: F, t15808: F, t15814: F, t15816: F, t15820: F, t15823: F, t15831: F, t1761: F, t3487: F, t3593: F, t3600: F, t3631: F, t4945: F, t498: F, t5060: F, t5089: F) -> F {
    let t15833 = -F::new(2.0) * t11613 * t1761 - t11925 * t1761 - t11928 * t1761 - F::new(2.0) * t1252 * t15820 + t15808 * t498 + t15814 * t498 + F::new(2.0) * t15816 * t498 + F::new(2.0) * t15823 * t498 + t15831 * t498 - F::new(2.0) * t3487 * t5089 + F::new(4.0) * t3593 * t5060 + F::new(2.0) * t3600 * t4945 - t3631 * t4945;
    t15833
}
