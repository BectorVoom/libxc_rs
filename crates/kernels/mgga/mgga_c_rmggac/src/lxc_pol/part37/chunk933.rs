//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 933/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk933<F: Float>(t76891: F, t68514: F, t68517: F, t74191: F, t74193: F, t74195: F, t74203: F, t15502: F, t495: F, t515: F, t7230: F, t7231: F) -> (F, F, F, F, F, F, F, F) {
    let t76892 = F::cast_from(0.68186654135613354322e-2_f64) * t76891;
    let t76893 = F::cast_from(0.81300399444200075499e-3_f64) * t68514;
    let t76894 = F::cast_from(0.81300399444200075499e-3_f64) * t68517;
    let t76896 = F::cast_from(0.10227998120342003148e-1_f64) * t74191;
    let t76897 = F::cast_from(0.25650144397517585626e-6_f64) * t74193;
    let t76898 = F::cast_from(0.25650144397517585626e-6_f64) * t74195;
    let t76904 = F::cast_from(0.23268647941669485538e-4_f64) * t74203;
    let t76912 = t7230 * t7231 * t515 * t15502 * t495;
    (t76892, t76893, t76894, t76896, t76897, t76898, t76904, t76912)
}
