//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 951/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk951<F: Float>(t74171: F, t74173: F, t74175: F, t74177: F, t74180: F, t14588: F, t623: F, t2147: F, t68514: F, t68517: F, t74191: F, t74193: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t76884 = F::new(0.1276937996798935182e-4) * t74171;
    let t76885 = F::new(0.2553875993597870364e-4) * t74173;
    let t76886 = F::new(0.3830813990396805546e-4) * t74175;
    let t76887 = F::new(0.1276937996798935182e-4) * t74177;
    let t76888 = F::new(0.1276937996798935182e-4) * t74180;
    let t76890 = t623 * t14588;
    let t76891 = t76890 * t2147;
    let t76892 = F::new(0.68186654135613354322e-2) * t76891;
    let t76893 = F::new(0.81300399444200075499e-3) * t68514;
    let t76894 = F::new(0.81300399444200075499e-3) * t68517;
    let t76896 = F::new(0.10227998120342003148e-1) * t74191;
    let t76897 = F::new(0.25650144397517585626e-6) * t74193;
    (t76884, t76885, t76886, t76887, t76888, t76892, t76893, t76894, t76896, t76897)
}
