//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 846/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk846<F: Float>(t1761: F, t19232: F, t19234: F, t19249: F, t2155: F, t24587: F, t27401: F, t27406: F, t27830: F, t29667: F, t29699: F, t29825: F, t4945: F, t8006: F, t8015: F, t8061: F, t8088: F) -> F {
    let t29827 = F::new(4.0) * t4945 * t8061 - t19249 * t2155 - t24587 - t19232 * t2155 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t8015 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t8006 - F::new(2.0) * t19234 * t2155 - F::new(2.0) * t4945 * t8088 - F::cast_from(0.18277045187202515961e-2_f64) * t27401 - F::new(2.0) * t27830 * t1761 + t29667 + t29699 + t29825;
    t29827
}
