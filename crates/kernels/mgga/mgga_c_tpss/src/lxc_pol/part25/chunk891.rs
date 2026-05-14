//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 891/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk891<F: Float>(t11526: F, t925: F, t8491: F, t926: F, t242: F, t2751: F, t3758: F, t967: F, t2685: F, t3916: F, t1464: F, t948: F, t345: F, t836: F, t2724: F, t3962: F, t8983: F) -> (F, F, F, F, F, F, F, F) {
    let t11528 = t925 * t11526 / 324.0;
    let t11535 = t926 * t8491;
    let t11548 = t242 * t2751 * t3758;
    let t11550 = t967 * t11548 / 3456.0;
    let t11562 = t2685 * t3916 / 162.0;
    let t11568 = t1464 * t948;
    let t11569 = t345 * t836;
    let t11575 = t1464 * t2724;
    let t11584 = t8983 * t3962;
    (t11528, t11535, t11550, t11562, t11568, t11569, t11575, t11584)
}
