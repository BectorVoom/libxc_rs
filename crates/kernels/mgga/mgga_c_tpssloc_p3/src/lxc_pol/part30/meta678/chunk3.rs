//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2123/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2123<F: Float>(t28030: F, t6535: F, t26114: F, t7461: F, t19994: F, t24995: F, t8945: F, t1266: F, t1393: F, t1459: F, t1774: F, t1849: F, t19450: F, t19451: F, t1976: F, t20127: F, t22461: F, t24999: F, t26098: F, t26138: F, t27993: F, t28020: F, t4037: F, t4072: F, t4077: F, t5494: F, t574: F, t6517: F, t652: F, t6539: F, t7670: F, t96355: F, t96358: F, t96360: F, t96361: F, t96682: F, t96732: F) -> F {
    let t96738 = F::new(2.0) * t28030 * t6535;
    let t96740 = F::new(4.0) * t26114 * t7461;
    let t96746 = F::new(6.0) * t24995 * t8945 * t19994;
    let t96749 = -t19450 * t1976 + t96355 - t96358 - t96360 - F::new(4.0) * t96361 * t1459 - F::new(4.0) * t24999 * t4037 - F::new(4.0) * t24999 * t4077 - F::new(2.0) * t6517 * t20127 - F::new(4.0) * t652 * t7670 * t4072 - F::new(2.0) * t26098 * t1774 + t28020 * t1393 + (t96682 + t96732) * t574 - F::new(2.0) * t19451 * t6539 - t96738 - t96740 - t27993 * t1266 + F::new(2.0) * t26138 * t1849 + t96746 - F::new(2.0) * t22461 * t5494;
    t96749
}
