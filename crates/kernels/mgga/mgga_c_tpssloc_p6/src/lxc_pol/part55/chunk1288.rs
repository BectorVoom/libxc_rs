//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1288/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1288<F: Float>(t34353: F, t3640: F, t118229: F, t118233: F, t118251: F, t125182: F, t125237: F, t125280: F, t1254: F, t125580: F, t1256: F, t125624: F, t125668: F, t125712: F, t125752: F, t1763: F, t193: F, t24905: F, t24909: F, t27834: F, t27843: F, t32555: F, t32561: F, t336: F, t4700: F, t5091: F, t7394: F, t7398: F, t8090: F) -> F {
    let t125759 = t34353 * t3640;
    let t125789 = t193 * t336 * (t125182 + t125237 + t125280 + t125580 + t125624 + t125668 + t125712 + t125752) * t1256 - t4700 * t125759 * t1254 - t4700 * t118229 * t1763 + F::cast_from(2.0_f64) * t4700 * t118233 * t27843 - t4700 * t32555 * t5091 - F::cast_from(2.0_f64) * t4700 * t24905 * t8090 + F::cast_from(4.0_f64) * t4700 * t24909 * t8090 * t1254 - F::cast_from(2.0_f64) * t4700 * t7398 * t27834 + F::cast_from(4.0_f64) * t4700 * t24909 * t1763 * t7394 - F::cast_from(6.0_f64) * t4700 * t118251 * t27843 + F::cast_from(2.0_f64) * t4700 * t32561 * t5091;
    t125789
}
