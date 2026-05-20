//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 979/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk979<F: Float>(t13283: F, t1512: F, t1516: F, t16872: F, t16976: F, t20904: F, t20908: F, t20938: F, t20944: F, t20949: F, t20953: F, t249: F, t4172: F, t5587: F, t5624: F, t5628: F, t817: F, t843: F, t9559: F, t9974: F) -> F {
    let t20958 = -t9974 * t20904 / F::new(512.0) - t843 * t20908 / F::new(768.0) + F::new(5.0) / F::new(256.0) * t4172 * t5624 - t16976 * t1516 / F::new(256.0) - t4172 * t5628 / F::new(256.0) + t20938 * t249 / F::new(3072.0) + t13283 * t5587 / F::new(512.0) - t9559 * t20944 / F::new(4.0) + F::new(5.0) / F::new(256.0) * t843 * t20949 - t817 * t20953 / F::new(3072.0) - t16872 * t1512 / F::new(1024.0);
    t20958
}
