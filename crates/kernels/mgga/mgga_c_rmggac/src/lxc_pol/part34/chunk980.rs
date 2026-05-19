//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 980/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk980<F: Float>(t74867: F, t74873: F, t74891: F, t74896: F, t74901: F, t74903: F, t74909: F, t74913: F, t74915: F, t74917: F, t74921: F, t15489: F, t16156: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77246 = F::cast_from(0.638468998399467591e-4_f64) * t74867;
    let t77247 = F::cast_from(0.81823984962736025184e-1_f64) * t74873;
    let t77249 = F::cast_from(0.85129199786595678799e-5_f64) * t74891;
    let t77250 = F::cast_from(0.85129199786595678799e-5_f64) * t74896;
    let t77251 = F::cast_from(0.85129199786595678799e-5_f64) * t74901;
    let t77252 = F::cast_from(0.2553875993597870364e-4_f64) * t74903;
    let t77253 = F::cast_from(0.2553875993597870364e-4_f64) * t74909;
    let t77254 = F::cast_from(0.2553875993597870364e-4_f64) * t74913;
    let t77255 = F::cast_from(0.2553875993597870364e-4_f64) * t74915;
    let t77256 = F::cast_from(0.79828278012425390427e-1_f64) * t74917;
    let t77258 = F::cast_from(0.10227998120342003148e-1_f64) * t74921;
    let t77259 = t16156 * t15489;
    (t77246, t77247, t77249, t77250, t77251, t77252, t77253, t77254, t77255, t77256, t77258, t77259)
}
