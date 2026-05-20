//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3014/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3014<F: Float>(t1022: F, t1058: F, t1060: F, t11034: F, t11046: F, t11054: F, t11059: F, t11065: F, t11066: F, t14577: F, t14587: F, t14630: F, t14651: F, t18047: F, t18080: F, t18086: F, t18093: F, t18107: F, t18121: F, t18162: F, t3120: F, t3180: F, t3186: F, t3193: F, t3200: F, t43480: F, t4669: F, t4677: F, t4681: F, t5928: F, t5929: F, t5932: F, t5936: F) -> F {
    let t63095 = F::new(2.0) * t18086 * t3193 + F::new(12.0) * t11059 * t5932 * t14577 + F::new(4.0) * t11034 * t18121 + F::new(2.0) * t1058 * t18047 * t1022 * t1060 + F::new(2.0) * t43480 * t5929 + F::new(4.0) * t14651 * t4681 - F::new(4.0) * t3200 * t4677 * t18107 - F::new(6.0) * t11065 * t5928 * t11066 * t3120 + F::new(2.0) * t3180 * t18162 + F::new(4.0) * t4669 * t14587 + F::new(2.0) * t3186 * t5936 * t11054 + F::new(2.0) * t11046 * t18080 * t18093 + t11046 * t5936 * t14630;
    t63095
}
