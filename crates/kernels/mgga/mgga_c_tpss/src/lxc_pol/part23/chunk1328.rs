//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1328/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1328<F: Float>(t5: F, t67934: F, t67964: F, t67990: F, t68019: F, t68049: F, t68091: F, t68118: F, t68146: F, t117: F, t2061: F, t6479: F, t116: F, t20785: F, t5983: F, t645: F, t1864: F, t2105: F) -> (F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t68150 = piecewise3(t8, 0.0, t67934 + t67964 + t67990 + t68019 + t68049 + t68091 + t68118 + t68146);
    let t68151 = t68150 * t117;
    let t68152 = t6479 * t2061;
    let t68156 = t20785 * t116;
    let t68163 = t5983 * t645;
    let t68168 = t1864 * t2105;
    (t68151, t68152, t68156, t68163, t68168)
}
