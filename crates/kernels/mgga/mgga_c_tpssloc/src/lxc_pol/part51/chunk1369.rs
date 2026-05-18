//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1369/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1369<F: Float>(t15868: F, t1983: F, t8640: F, t121019: F, t121129: F, t121132: F, t121134: F, t121136: F, t121138: F, t121142: F, t23938: F, t26898: F, t26902: F, t32674: F, t32676: F, t32679: F, t510: F, t7472: F, t8450: F) -> F {
    let t121144 = t1983 * t8640 * t15868;
    let t121149 = -t121129 * t510 - F::new(2.0) * t23938 * t7472 + F::new(3.0) * t26898 * t8450 - t26902 * t8450 - t121019 + t121132 - t121134 - t121136 - t121138 + t121142 - t121144 - t32674 - t32676 - t32679;
    t121149
}
