//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 807/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk807<F: Float>(t74590: F, t15616: F, t2106: F, t2145: F, t14683: F, t8577: F, t71021: F, t68742: F, t3219: F, t38351: F, t38355: F, t14639: F, t8571: F, t15457: F, t16156: F, t74650: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77113 = 0.2627895913935205078e-5 * t74590;
    let t77116 = t2145 * t15616 * t2106;
    let t77117 = 0.90915538847484472429e-2 * t77116;
    let t77118 = t8577 * t14683;
    let t77119 = 0.42564599893297839398e-5 * t77118;
    let t77121 = 0.21684485328539747656e-4 * t71021;
    let t77123 = 0.79828278012425390427e-1 * t68742;
    let t77124 = t38351 * t3219;
    let t77125 = 0.42564599893297839398e-5 * t77124;
    let t77126 = t38355 * t3219;
    let t77127 = 0.42564599893297839398e-5 * t77126;
    let t77128 = t8571 * t14639;
    let t77129 = 0.42564599893297839398e-5 * t77128;
    let t77131 = t16156 * t15457;
    let t77132 = 0.29795219925308487578e-4 * t77131;
    let t77134 = 0.2627895913935205078e-5 * t74650;
    (t77113, t77117, t77119, t77121, t77123, t77125, t77127, t77129, t77132, t77134)
}
