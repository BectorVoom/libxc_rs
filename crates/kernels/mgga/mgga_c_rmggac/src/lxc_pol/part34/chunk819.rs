//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 819/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk819<F: Float>(t71021: F, t68742: F, t3219: F, t38351: F, t38355: F, t14639: F, t8571: F, t15457: F, t16156: F, t74650: F, t74652: F, t74657: F, t68753: F, t68739: F, t71031: F, t74609: F, t74610: F, t74616: F, t74647: F, t74655: F) -> (F,) {
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
    let t77135 = 0.12263514265030957031e-4 * t74652;
    let t77137 = 0.54549323308490683456e-1 * t74657;
    let t77138 = 0.54549323308490683456e-1 * t68753;
    let t77139 = -t74609 + t77121 - 0.31062809106223861415e-2 * t74610 + t68739 + t77123 - t77125 - t77127 - t77129 - 0.49700494569958178264e-1 * t74616 - t77132 + t71031 + 0.58171619854173713846e-5 * t74647 + t77134 - t77135 + 0.4379826523225341797e-6 * t74655 + t77137 + t77138;
    (t77139,)
}
