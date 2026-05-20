//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2367/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2367<F: Float>(t15857: F, t1873: F, t652: F, t1874: F, t45632: F, t12841: F, t1774: F, t1849: F, t22461: F, t22559: F, t2320: F, t23855: F, t4037: F, t510: F, t6517: F, t7670: F, t90352: F, t91752: F, t91755: F, t91757: F, t91759: F, t91762: F, t91763: F, t91765: F, t91767: F, t91769: F, t91771: F, t91777: F) -> F {
    let t91780 = F::new(2.0) * t652 * t15857 * t1873;
    let t91782 = F::new(2.0) * t45632 * t1874;
    let t91789 = -F::new(2.0) * t12841 * t6517 - t1774 * t22559 + t1849 * t23855 - F::new(4.0) * t22461 * t4037 - F::new(2.0) * t2320 * t7670 - F::new(2.0) * t510 * t90352 - t91752 - t91755 - t91757 - t91759 - t91762 - t91763 - t91765 - t91767 + t91769 - t91771 - t91777 - t91780 - t91782;
    t91789
}
