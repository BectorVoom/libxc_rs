//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1466/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1466<F: Float>(t11668: F, t11678: F, t11692: F, t15569: F, t15740: F, t1653: F, t19080: F, t22158: F, t22312: F, t3578: F, t45114: F, t52680: F, t5971: F, t5975: F, t6221: F, t6225: F, t6230: F, t65819: F, t72512: F, t72530: F, t72542: F, t72556: F, t72560: F) -> F {
    let t79087 = F::new(5.0) / F::new(1152.0) * t15740 * t22158 + F::new(5.0) / F::new(1728.0) * t72512 + t45114 * t3578 * t22312 * t1653 / F::new(192.0) - t72530 / F::new(288.0) - t52680 / F::new(3888.0) - t11678 * t3578 * t6225 * t5975 / F::new(192.0) + t72542 / F::new(54.0) + F::new(5.0) / F::new(1152.0) * t11678 * t11668 * t6225 * t5971 + t65819 / F::new(1728.0) - F::new(5.0) / F::new(216.0) * t15569 * t22158 + t11692 * t3578 * t6230 * t5975 / F::new(384.0) - t19080 * t6221 / F::new(48.0) - t72556 / F::new(576.0) + F::new(5.0) / F::new(864.0) * t72560;
    t79087
}
