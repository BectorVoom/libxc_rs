//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1551/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1551<F: Float>(t1060: F, t18154: F, t17959: F, t381: F, t1003: F, t1058: F, t1063: F, t14608: F, t1610: F, t1632: F, t17876: F, t18129: F, t18131: F, t18139: F, t18142: F, t18151: F, t3180: F, t3186: F, t3200: F, t353: F, t384: F, t4615: F, t4669: F, t4678: F, t4681: F, t4685: F, t4689: F, t4691: F, t5903: F, t5933: F, t5941: F) -> (F, F) {
    let t18155 = t18154 * t1060;
    let t18161 = t381 * t17959;
    let t18162 = t18161 * t1060;
    let t18164 = t1003 * t5941 + F::new(2.0) * t1058 * t18151 + t1058 * t18155 + t1058 * t18162 + t1063 * t5903 - F::new(2.0) * t14608 * t4685 + F::new(2.0) * t1610 * t4691 + F::new(2.0) * t1632 * t4615 + t17876 * t384 + t18129 * t353 - F::new(2.0) * t18131 * t3200 + F::new(4.0) * t18139 * t3186 + F::new(4.0) * t18142 * t3186 + F::new(2.0) * t3180 * t5933 + F::new(2.0) * t4669 * t4678 + F::new(2.0) * t4669 * t4681 + F::new(2.0) * t4669 * t4689;
    (t18161, t18164)
}
