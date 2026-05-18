//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1208/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1208<F: Float>(t102948: F, t107250: F, t107260: F, t107265: F, t1375: F, t1842: F, t1843: F, t20662: F, t2092: F, t29299: F, t29360: F, t29372: F, t3887: F, t5215: F, t5321: F, t7194: F, t74860: F, t84659: F) -> F {
    let t107772 = -F::new(3.0) * t102948 * t1843 - t84659 - F::new(0.9869604401089358619e-1) * t107250 - F::new(3.0) * t74860 * t2092 + F::new(6.0) * t5215 * t29372 + F::new(6.0) * t5321 * t29372 + F::new(0.16449340668482264365e-1) * t107260 - t7194 * t20662 + F::new(6.0) * t1375 * t3887 * t29360 * t1842 - F::new(18.0) * t5321 * t29299 + F::new(0.29608813203268075857e0) * t107265;
    t107772
}
