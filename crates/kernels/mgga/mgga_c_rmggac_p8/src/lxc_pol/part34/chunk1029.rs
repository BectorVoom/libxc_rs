//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1029/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1029<F: Float>(t76151: F, t76154: F, t76159: F, t71863: F, t71871: F, t71892: F, t76173: F, t76161: F, t76163: F, t76165: F, t76167: F, t76169: F, t76171: F) -> F {
    let t77848 = F::cast_from(0.40911992481368012595e-1_f64) * t76151;
    let t77849 = F::cast_from(0.5454932330849068346e-1_f64) * t76154;
    let t77850 = F::cast_from(0.40911992481368012595e-1_f64) * t76159;
    let t77851 = F::cast_from(0.18183107769496894486e-1_f64) * t71863;
    let t77852 = F::cast_from(0.36366215538993788972e-1_f64) * t71871;
    let t77853 = F::cast_from(0.27274661654245341729e-1_f64) * t71892;
    let t77860 = F::cast_from(0.20455996240684006296e-1_f64) * t76173;
    let t77861 = -t77848 + t77849 + t77850 + t77851 + t77852 - t77853 - F::cast_from(0.18637685463734316849e-1_f64) * t76161 + F::cast_from(0.46594213659335792122e-1_f64) * t76163 + F::cast_from(0.93188427318671584245e-2_f64) * t76165 + F::cast_from(0.46594213659335792124e-1_f64) * t76167 - F::cast_from(0.93188427318671584248e-1_f64) * t76169 - F::cast_from(0.15531404553111930708e-1_f64) * t76171 - t77860;
    t77861
}
