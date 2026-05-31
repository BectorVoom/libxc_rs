//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1060/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1060<F: Float>(t2685: F, t3916: F, t1407: F, t2704: F, t2741: F, t1464: F, t948: F, t345: F, t836: F, t581: F, t2724: F, t3962: F, t8983: F) -> (F, F, F, F, F) {
    let t11562 = t2685 * t3916 / F::cast_from(162.0_f64);
    let t11564 = t1407 * t2704;
    let t11565 = t2741 * t11564;
    let t11568 = t1464 * t948;
    let t11569 = t345 * t836;
    let t11570 = t11569 * t581;
    let t11571 = t11568 * t11570;
    let t11572 = t2741 * t11571;
    let t11575 = t1464 * t2724;
    let t11577 = t948 * t836 * t581;
    let t11578 = t11575 * t11577;
    let t11579 = t2741 * t11578;
    let t11584 = t8983 * t3962;
    (t11562, t11565, t11572, t11579, t11584)
}
