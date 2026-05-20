//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1304/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1304<F: Float>(t2205: F, t6483: F, t1858: F, t8283: F, t30616: F, t576: F, t111302: F, t111308: F, t111310: F, t111312: F, t112051: F, t1404: F, t1852: F, t20186: F, t2206: F, t3: F, t30395: F, t30582: F, t5364: F, t5381: F, t580: F, t8284: F, t8299: F) -> F {
    let t112083 = t2205 * t6483;
    let t112084 = t8283 * t1858;
    let t112087 = t576 * t30616;
    let t112090 = t112051 * t3 * t580 + t1404 * t30582 + F::new(2.0) * t1852 * t30395 + t20186 * t2206 + F::new(2.0) * t5364 * t8299 + F::new(2.0) * t5381 * t8284 + t111302 + t111308 + t111310 + t111312 + t112083 + F::new(2.0) * t112084 + t112087;
    t112090
}
