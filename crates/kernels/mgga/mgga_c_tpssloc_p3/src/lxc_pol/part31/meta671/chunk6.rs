//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2007/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2007<F: Float>(t5: F, t102145: F, t102171: F, t102198: F, t102223: F, t102252: F, t102278: F, t102284: F, t102305: F, t112: F, t19450: F, t19577: F, t19596: F, t1983: F, t19994: F, t20098: F, t20109: F, t2040: F, t2075: F, t2079: F, t22574: F, t23938: F, t24432: F, t24987: F, t24995: F, t26898: F, t26977: F, t27144: F, t27145: F, t28821: F, t29222: F, t33899: F, t510: F, t5161: F, t5460: F, t6876: F, t7042: F, t7170: F, t7171: F, t7217: F, t74032: F, t75203: F, t75560: F, t7685: F, t7904: F, t9016: F, t96824: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t102309 = piecewise3::<F>(t8, F::new(0.0), t102145 + t102171 + t102198 + t102223 + t102252 + t102278 + t102284 + t102305);
    let t102310 = t102309 * t112;
    let t102320 = F::new(3.0) * t28821 * t7171 + F::new(6.0) * t24995 * t9016 * t19994 - t19450 * t2075 + F::new(3.0) * t1983 * t7170 * t96824 + F::new(6.0) * t7685 * t26898 - t1983 * t7217 * t19596 - F::new(6.0) * t22574 * t33899 * t19577 - F::new(2.0) * t1983 * t27144 * t5161 + F::new(2.0) * t7685 * t27145 - t6876 * t29222 - F::new(4.0) * t23938 * t5460 - F::new(4.0) * t26977 * t5460 - F::new(4.0) * t7042 * t20109 - F::new(6.0) * t24995 * t24432 * t75203 - t102310 * t510 + t2079 * t20098 + F::new(6.0) * t24987 * t7904 - F::new(3.0) * t22574 * t24432 * t74032 - F::new(2.0) * t75560 * t2040;
    (t102310, t102320)
}
