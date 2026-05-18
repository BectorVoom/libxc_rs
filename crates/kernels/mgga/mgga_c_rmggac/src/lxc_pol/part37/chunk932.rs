//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 932/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk932<F: Float>(t74102: F, t74161: F, t74163: F, t70867: F, t74171: F, t74173: F, t74175: F, t74177: F, t74180: F, t14588: F, t623: F, t2147: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t76859 = F::new(0.2553875993597870364e-4) * t74102;
    let t76878 = F::new(0.23268647941669485538e-4) * t74161;
    let t76879 = F::new(0.11634323970834742769e-3) * t74163;
    let t76880 = F::new(0.29795219925308487579e-4) * t70867;
    let t76884 = F::new(0.1276937996798935182e-4) * t74171;
    let t76885 = F::new(0.2553875993597870364e-4) * t74173;
    let t76886 = F::new(0.3830813990396805546e-4) * t74175;
    let t76887 = F::new(0.1276937996798935182e-4) * t74177;
    let t76888 = F::new(0.1276937996798935182e-4) * t74180;
    let t76890 = t623 * t14588;
    let t76891 = t76890 * t2147;
    (t76859, t76878, t76879, t76880, t76884, t76885, t76886, t76887, t76888, t76891)
}
